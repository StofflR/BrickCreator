from __future__ import annotations

import argparse
import json
import re
import struct
from pathlib import Path
from typing import Iterable


def iter_density_dirs(root: Path) -> list[Path]:
    """
    Return all Android drawable density directories under `root`.

    Accepts either:
    - `root` itself containing `drawable-*` dirs, or
    - a typical Android `res/` folder containing those dirs.
    """
    candidates = [root, root / "res"]
    for base in candidates:
        if not base.exists() or not base.is_dir():
            continue
        dirs = sorted(
            [
                p
                for p in base.iterdir()
                if p.is_dir()
                and p.name.startswith("drawable")
                and (p.name == "drawable" or p.name.startswith("drawable-"))
            ],
            key=lambda p: p.name,
        )
        if dirs:
            return dirs
    return []


def read_png_dimensions(path: Path) -> tuple[int, int]:
    with path.open("rb") as f:
        header = f.read(24)
    if len(header) < 24 or header[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("Not a PNG file")
    width, height = struct.unpack(">II", header[16:24])
    return int(width), int(height)


_ORDNER_RE = re.compile(r"^\s*Type:\s*(?P<path>.+?)\s*$")
_SIZE_RE = re.compile(r"^(?P<name>\S+)\s+(?P<w>\d+)x(?P<h>\d+)\s*$")


def _density_key_from_folder(folder: str) -> str:
    """
    Turn an 'Ordner: /path/to/drawable-ldpi' line into a short density key like 'ldpi'.
    Falls back to the folder basename if it doesn't look like a drawable density dir.
    """
    base = Path(folder.strip()).name
    if base == "drawable":
        return "drawable"
    if base.startswith("drawable-"):
        return base.removeprefix("drawable-")
    return base


def parse_png_sizes_txt(path: Path) -> dict[str, dict[str, tuple[int, int]]]:
    """
    Parse utils/png_sizes.txt format into:
      density_key -> filename -> (width, height)
    """
    if not path.exists():
        raise FileNotFoundError(path)

    result: dict[str, dict[str, tuple[int, int]]] = {}
    current_density: str | None = None

    for raw in path.read_text(encoding="utf-8").splitlines():
        if not raw.strip():
            continue

        m = _ORDNER_RE.match(raw)
        if m:
            current_density = _density_key_from_folder(m.group("path"))
            result.setdefault(current_density, {})
            continue

        if current_density is None:
            continue

        line = raw.strip()
        m = _SIZE_RE.match(line)
        if not m:
            continue

        name = m.group("name")
        w = int(m.group("w"))
        h = int(m.group("h"))
        result[current_density][name] = (w, h)

    return result


def _mode_size(sizes: Iterable[tuple[int, int]]) -> tuple[int, int] | None:
    counts: dict[tuple[int, int], int] = {}
    for s in sizes:
        counts[s] = counts.get(s, 0) + 1
    if not counts:
        return None
    return max(counts.items(), key=lambda kv: kv[1])[0]


def _pick_named(
    files: dict[str, tuple[int, int]], names: list[str]
) -> tuple[str, tuple[int, int]] | None:
    for n in names:
        if n in files:
            return n, files[n]
    return None


def _pick_base_size(
    files: dict[str, tuple[int, int]], height_units: int
) -> tuple[str, tuple[int, int]] | None:
    preferred = [
        f"brick_{height_units}h_gold.9.png",
        f"brick_{height_units}h_yellow.9.png",
        f"brick_{height_units}h_blue.9.png",
    ]
    picked = _pick_named(files, preferred)
    if picked is not None:
        return picked

    prefix = f"brick_{height_units}h_"
    candidates = [
        (name, size)
        for name, size in files.items()
        if name.startswith(prefix)
        and name.endswith(".png")
        and "control" not in name
        and "_when_" not in name
        and "define" not in name
    ]
    mode = _mode_size([size for _, size in candidates])
    if mode is None:
        return None

    for name, size in sorted(candidates, key=lambda x: x[0]):
        if size == mode:
            return name, size
    return None


def summarize_brick_sizes(
    parsed: dict[str, dict[str, tuple[int, int]]],
) -> dict[str, dict[str, object]]:
    """
    For each density, compute brick reference sizes:
      h1_base, h2_base, h3_base, h1_control, h2_control
    """
    summary: dict[str, dict[str, object]] = {}
    for density, files in parsed.items():
        h1_base = _pick_base_size(files, 1)
        h2_base = _pick_base_size(files, 2)
        h3_base = _pick_base_size(files, 3)

        h1_control = _pick_named(
            files, ["brick_control_1h.9.png", "brick_control_gold_1h.9.png"]
        )
        h2_control = _pick_named(
            files, ["brick_control_2h.9.png", "brick_control_gold_2h.9.png"]
        )

        def pack(picked: tuple[str, tuple[int, int]] | None) -> dict[str, object] | None:
            if picked is None:
                return None
            name, size = picked
            return {"file": name, "size": size}

        summary[density] = {
            "h1_base": pack(h1_base),
            "h2_base": pack(h2_base),
            "h3_base": pack(h3_base),
            "h1_control": pack(h1_control),
            "h2_control": pack(h2_control),
        }
    return summary


def _density_sort_key(density: str) -> tuple[int, str]:
    order = {
        "ldpi": 10,
        "mdpi": 20,
        "hdpi": 30,
        "xhdpi": 40,
        "xxhdpi": 50,
        "xxxhdpi": 60,
        "tvdpi": 70,
        "anydpi": 80,
        "nodpi": 90,
        "drawable": 95,
    }
    return (order.get(density, 999), density)


def _format_size(size: tuple[int, int] | None) -> str:
    if size is None:
        return "—"
    return f"{size[0]}x{size[1]}"


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Summarize brick PNG sizes from utils/png_sizes.txt. "
            "Use --write-sizes to (re)generate png_sizes.txt from drawable-* folders."
        )
    )
    repo_root = Path(__file__).resolve().parent.parent
    stoffl_root = repo_root.parent

    def pick_default_root() -> Path:
        candidates = [
            repo_root / "ref_catroid_ninepatch",
            stoffl_root / "ref_catroid_ninepatch",
            stoffl_root,
        ]
        for cand in candidates:
            if cand.exists() and any(iter_density_dirs(cand)):
                return cand
        return repo_root / "ref_catroid_ninepatch"

    default_root = pick_default_root()
    parser.add_argument(
        "--scan-root",
        default=str(default_root),
        help="Root folder containing drawable-* (default: auto-detected)",
    )
    parser.add_argument(
        "--write-sizes",
        action="store_true",
        help="Scan drawable-* and write png_sizes.txt (legacy behavior).",
    )
    parser.add_argument(
        "--input",
        default=str(Path(__file__).parent / "png_sizes.txt"),
        help="Input .txt path to read (default: utils/png_sizes.txt)",
    )
    parser.add_argument(
        "--output",
        default=str(Path(__file__).parent / "png_sizes.txt"),
        help="Output .txt path when --write-sizes is used (default: utils/png_sizes.txt)",
    )
    parser.add_argument(
        "--format",
        choices=["text", "json"],
        default="text",
        help="Summary output format (default: text).",
    )
    parser.add_argument(
        "--all-densities",
        action="store_true",
        help="Print canonical density list even if missing in input (default: only densities present).",
    )

    args = parser.parse_args()
    if args.write_sizes:
        root = Path(args.scan_root)
        if not root.exists():
            raise SystemExit(f"Root folder not found: {root}")

        density_dirs = iter_density_dirs(root)
        if not density_dirs:
            raise SystemExit(f"No drawable-* folders found in: {root}")

        output_file = Path(args.output)
        output_file.parent.mkdir(parents=True, exist_ok=True)
        with output_file.open("w", encoding="utf-8", newline="\n") as out:
            for density_dir in density_dirs:
                out.write(f"\nType: {density_dir.name}\n")

                for path in sorted(density_dir.iterdir(), key=lambda p: p.name):
                    if not path.is_file():
                        continue
                    if path.suffix.lower() != ".png":
                        continue

                    try:
                        w, h = read_png_dimensions(path)
                        out.write(f"{path.name} {w}x{h}\n")
                    except Exception as e:
                        out.write(f"{path.name} ERROR: {e}\n")
        return 0

    parsed = parse_png_sizes_txt(Path(args.input))
    summary = summarize_brick_sizes(parsed)

    if args.format == "json":
        print(json.dumps(summary, indent=2, ensure_ascii=False))
        return 0

    if args.all_densities:
        canonical_densities = [
            "ldpi",
            "mdpi",
            "tvdpi",
            "hdpi",
            "xhdpi",
            "xxhdpi",
            "xxxhdpi",
            "anydpi",
            "nodpi",
            "drawable",
        ]
        extra_densities = sorted(
            [d for d in summary.keys() if d not in canonical_densities], key=_density_sort_key
        )
        densities_to_print = canonical_densities + extra_densities
    else:
        densities_to_print = sorted(summary.keys(), key=_density_sort_key)

    for density in densities_to_print:
        s = summary.get(density, {})
        print(f"\nDensity: {density}")
        for key in ["h1_base", "h2_base", "h3_base", "h1_control", "h2_control"]:
            entry = s.get(key)
            if not isinstance(entry, dict):
                print(f"  {key}: —")
                continue
            print(f"  {key}: {_format_size(entry.get('size'))} ({entry.get('file')})")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())