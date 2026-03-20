from pathlib import Path
import json

source_dir = Path("../ref_all_bricks") # source folder that contains the bricks in the old json format
target_dir = Path(__file__).parent / "output"

offset_map = {
    "HOCollapse": [0.15, 0.0],
    "H1Base": [0.15, 0.13],
    "H1Control": [0.15, 0.22],
    "H2Base": [0.15, 0.09],
    "H2Control": [0.15, 0.17],
    "H3Base": [0.15, 0.03],
}

for json_file in source_dir.rglob("*.json"):
    relative_path = json_file.relative_to(source_dir)

    new_path = target_dir / relative_path
    print(new_path)
    new_path.parent.mkdir(parents=True, exist_ok=True)

    with open(json_file, "r", encoding="utf-8") as f:
        data = json.load(f)

    brick_type = data["Size"] + data["Type"]

    new_data = {
        "content": data.get("Content", ""),
        "brick_type": brick_type,
        "color_scheme": [
            data["Color"],
            data["Shade"],
            data["Border"],
            data["Text"]
        ],
        "offset": [
            data["X"] / data["Width"],
            data["Y"] / data["Width"]
        ],
        "scale": [18.75, 13.0]
    }

    offset_map.get(new_data["brick_type"])

    with open(new_path, "w", encoding="utf-8") as f:
        json.dump(new_data, f, indent=2, ensure_ascii=False)