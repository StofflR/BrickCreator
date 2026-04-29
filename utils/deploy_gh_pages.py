#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


DEFAULT_BRANCH = "gh-pages"
DEFAULT_PUBLIC_URL = "/BrickCreator/"
DEFAULT_REMOTE = "origin"
DEFAULT_COMMIT_MESSAGE = "Deploy GitHub Pages"


def run(
    command: list[str],
    *,
    cwd: Path,
    capture_output: bool = False,
) -> subprocess.CompletedProcess[str]:
    print(f"$ {' '.join(command)}")
    return subprocess.run(
        command,
        cwd=cwd,
        check=True,
        text=True,
        capture_output=capture_output,
    )


def git_output(repo_root: Path, *args: str) -> str:
    result = run(["git", *args], cwd=repo_root, capture_output=True)
    return result.stdout.strip()


def ensure_command_available(command: str) -> None:
    if shutil.which(command) is None:
        raise SystemExit(f"Required command not found in PATH: {command}")


def repo_root_from(script_path: Path) -> Path:
    return script_path.resolve().parent.parent


def remove_worktree_contents(worktree_dir: Path) -> None:
    for child in worktree_dir.iterdir():
        if child.name == ".git":
            continue
        if child.is_dir():
            shutil.rmtree(child)
        else:
            child.unlink()


def copy_dist_contents(dist_dir: Path, worktree_dir: Path) -> None:
    for child in dist_dir.iterdir():
        target = worktree_dir / child.name
        if child.is_dir():
            shutil.copytree(child, target, dirs_exist_ok=True)
        else:
            shutil.copy2(child, target)


def write_nojekyll(worktree_dir: Path) -> None:
    (worktree_dir / ".nojekyll").write_text("", encoding="utf-8")


def branch_exists(repo_root: Path, branch: str) -> bool:
    result = subprocess.run(
        ["git", "show-ref", "--verify", "--quiet", f"refs/heads/{branch}"],
        cwd=repo_root,
        text=True,
    )
    return result.returncode == 0


def remote_branch_exists(repo_root: Path, remote: str, branch: str) -> bool:
    result = subprocess.run(
        ["git", "ls-remote", "--exit-code", "--heads", remote, branch],
        cwd=repo_root,
        text=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0


def ensure_gh_pages_branch(repo_root: Path, remote: str, branch: str) -> None:
    if branch_exists(repo_root, branch):
        return

    if remote_branch_exists(repo_root, remote, branch):
        run(["git", "fetch", remote, f"{branch}:{branch}"], cwd=repo_root)
        return

    default_branch = git_output(repo_root, "rev-parse", "--abbrev-ref", "HEAD")
    run(["git", "branch", branch, default_branch], cwd=repo_root)


def build_frontend(
    repo_root: Path,
    frontend_dir: Path,
    public_url: str,
) -> Path:
    run(
        [
            "trunk",
            "build",
            "--release",
            "--config",
            str(frontend_dir / "Trunk.toml"),
            "--public-url",
            public_url,
        ],
        cwd=repo_root,
    )
    dist_dir = frontend_dir / "dist"
    if not dist_dir.exists():
        raise SystemExit(f"Build finished but dist directory was not found: {dist_dir}")
    return dist_dir


def deploy_dist(
    repo_root: Path,
    dist_dir: Path,
    remote: str,
    branch: str,
    commit_message: str,
) -> None:
    with tempfile.TemporaryDirectory(prefix="brickcreator-gh-pages-") as tmp_dir:
        worktree_dir = Path(tmp_dir) / branch
        run(["git", "worktree", "add", worktree_dir.as_posix(), branch], cwd=repo_root)
        try:
            run(["git", "pull", "--ff-only", remote, branch], cwd=worktree_dir)
            remove_worktree_contents(worktree_dir)
            copy_dist_contents(dist_dir, worktree_dir)
            write_nojekyll(worktree_dir)

            run(["git", "add", "--all"], cwd=worktree_dir)

            status = git_output(worktree_dir, "status", "--porcelain")
            if not status:
                print("No gh-pages changes to commit.")
                return

            run(["git", "commit", "-m", commit_message], cwd=worktree_dir)
            run(["git", "push", remote, branch], cwd=worktree_dir)
        finally:
            run(["git", "worktree", "remove", "--force", worktree_dir.as_posix()], cwd=repo_root)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Build the frontend with Trunk and deploy the generated files to the "
            "gh-pages branch for GitHub Pages."
        )
    )
    parser.add_argument(
        "--branch",
        default=DEFAULT_BRANCH,
        help=f"Git branch to publish to. Default: {DEFAULT_BRANCH}",
    )
    parser.add_argument(
        "--remote",
        default=DEFAULT_REMOTE,
        help=f"Git remote to push to. Default: {DEFAULT_REMOTE}",
    )
    parser.add_argument(
        "--public-url",
        default=DEFAULT_PUBLIC_URL,
        help=f"Public base URL passed to Trunk. Default: {DEFAULT_PUBLIC_URL}",
    )
    parser.add_argument(
        "--commit-message",
        default=DEFAULT_COMMIT_MESSAGE,
        help=f"Commit message for the deployment commit. Default: {DEFAULT_COMMIT_MESSAGE}",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    script_path = Path(__file__)
    repo_root = repo_root_from(script_path)
    frontend_dir = repo_root / "frontend"

    ensure_command_available("git")
    ensure_command_available("trunk")

    if not frontend_dir.exists():
        raise SystemExit(f"Frontend directory not found: {frontend_dir}")

    ensure_gh_pages_branch(repo_root, args.remote, args.branch)
    dist_dir = build_frontend(repo_root, frontend_dir, args.public_url)
    deploy_dist(repo_root, dist_dir, args.remote, args.branch, args.commit_message)

    page_url = f"https://stofflr.github.io{args.public_url}"
    print(f"Deployment finished. GitHub Pages URL: {page_url}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
