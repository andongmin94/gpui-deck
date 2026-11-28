"""Build the downloadable Cargo application from the checked-out billing block."""

import json
import os
import shutil
import subprocess
import tempfile
import tomllib
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

ROOT = Path(__file__).resolve().parents[2]
BLOCK = ROOT / "blocks/billing-dashboard"
OUTPUT = ROOT / "docs/public/downloads/billing-dashboard.zip"


def stage_app(app):
    manifest = json.loads((BLOCK / "block.json").read_text(encoding="utf-8"))
    if manifest["version"] != 1 or manifest["name"] != BLOCK.name:
        raise ValueError("expected the billing-dashboard v1 block manifest")
    module = manifest["name"].replace("-", "_")
    ui = app / "src/ui"
    block_dir = ui / module
    for file in manifest["files"]:
        source = (BLOCK / file["source"]).resolve()
        source.relative_to(BLOCK)
        target = (block_dir / file["target"]).resolve()
        target.relative_to(block_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)

    # Record the checkout as the same GitHub block used by the published add
    # command. The CLI then renders its module declarations and recipe closure.
    index = {
        module: {
            "url": f"https://raw.githubusercontent.com/andongmin94/gpui-deck/main/blocks/{BLOCK.name}/block.json",
            "files": sorted(file["target"] for file in manifest["files"]),
        }
    }
    (ui / ".gpui-deck-blocks.json").write_text(
        json.dumps(index, indent=2) + "\n", encoding="utf-8"
    )
    cargo_bin = Path.home() / ".cargo/bin"
    search_path = str(cargo_bin) + os.pathsep + os.environ["PATH"]
    cargo = shutil.which("cargo", path=search_path)
    if cargo is None:
        raise RuntimeError("install Cargo to build the billing dashboard download")
    subprocess.run(
        [
            cargo, "run", "--locked", "--quiet", "-p", "gpui-deck-cli", "--",
            "add", *manifest["recipes"], "--to", str(ui),
        ],
        cwd=ROOT,
        env={**os.environ, "PATH": search_path},
        check=True,
    )

    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]
    dependencies = (ROOT / "crates/cli/templates/dependencies.toml").read_text(encoding="utf-8")
    package = '[package]\nname = "billing-dashboard"\nversion = "0.1.0"\nedition = "2024"\n\n'
    (app / "Cargo.toml").write_text(
        package + dependencies.replace("GPUI_DECK_VERSION", workspace["package"]["version"]),
        encoding="utf-8",
    )
    for name in ("main.rs", "billing_window.rs"):
        shutil.copyfile(BLOCK / "app" / name, app / "src" / name)
    fonts = app / "assets/fonts"
    fonts.mkdir(parents=True)
    for name in (
        "Pretendard-Regular.otf", "Pretendard-Medium.otf", "Pretendard-SemiBold.otf", "LICENSE"
    ):
        shutil.copyfile(ROOT / "crates/base/examples/assets/fonts" / name, fonts / name)
    (app / ".gitignore").write_text("/target\n", encoding="utf-8")


def main():
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=OUTPUT.parent) as staging:
        stage = Path(staging)
        app = stage / "billing-dashboard"
        stage_app(app)
        archive = stage / OUTPUT.name
        with ZipFile(archive, "w", compression=ZIP_DEFLATED) as bundle:
            for path in sorted(app.rglob("*"), key=lambda path: path.relative_to(app).as_posix()):
                if path.is_file():
                    # Fixed ZIP metadata and LF give all builders the same sources.
                    info = ZipInfo(path.relative_to(stage).as_posix())
                    info.create_system = 3
                    info.external_attr = 0o100644 << 16
                    data = path.read_bytes()
                    if path.suffix in {".rs", ".toml", ".json"} or not path.suffix:
                        data = data.replace(b"\r\n", b"\n")
                    bundle.writestr(info, data, compress_type=ZIP_DEFLATED)
        archive.replace(OUTPUT)
    print(f"Built {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
