"""Build standalone Cargo projects from the checked-out GPUI Deck blocks."""

import json
import os
import shutil
import subprocess
import tempfile
import tomllib
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

ROOT = Path(__file__).resolve().parents[2]
BLOCKS = ROOT / "blocks"
DOWNLOADS = ROOT / "docs/public/downloads"


def stage_app(app, block):
    manifest = json.loads((block / "block.json").read_text(encoding="utf-8"))
    if manifest["version"] != 1 or manifest["name"] != block.name:
        raise ValueError(f"expected the {block.name} v1 block manifest")
    module = manifest["name"].replace("-", "_")
    ui = app / "src/ui"
    block_dir = ui / module
    for file in manifest["files"]:
        source = (block / file["source"]).resolve()
        source.relative_to(block)
        target = (block_dir / file["target"]).resolve()
        target.relative_to(block_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)

    # Record the checkout as the same GitHub block used by the published add
    # command. The CLI then renders its module declarations and recipe closure.
    index = {
        module: {
            "url": f"https://raw.githubusercontent.com/andongmin94/gpui-deck/main/blocks/{block.name}/block.json",
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
        raise RuntimeError("install Cargo to build the downloadable block apps")
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
    package = f'[package]\nname = "{block.name}"\nversion = "0.1.0"\nedition = "2024"\n\n'
    (app / "Cargo.toml").write_text(
        package + dependencies.replace("GPUI_DECK_VERSION", workspace["package"]["version"]),
        encoding="utf-8",
    )
    for name in ("main.rs", "block_window.rs"):
        shutil.copyfile(BLOCKS / "starter" / name, app / "src" / name)
    shutil.copyfile(block / "app/screen.rs", app / "src/screen.rs")
    fonts = app / "assets/fonts"
    fonts.mkdir(parents=True)
    for name in (
        "Pretendard-Regular.otf", "Pretendard-Medium.otf", "Pretendard-SemiBold.otf", "LICENSE"
    ):
        shutil.copyfile(ROOT / "crates/base/examples/assets/fonts" / name, fonts / name)
    (app / ".gitignore").write_text("/target\n", encoding="utf-8")


def build_archive(block):
    output = DOWNLOADS / f"{block.name}.zip"
    with tempfile.TemporaryDirectory(dir=DOWNLOADS) as staging:
        stage = Path(staging)
        app = stage / block.name
        stage_app(app, block)
        archive = stage / output.name
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
        archive.replace(output)
    print(f"Built {output.relative_to(ROOT)}")


if __name__ == "__main__":
    DOWNLOADS.mkdir(parents=True, exist_ok=True)
    for manifest in sorted(BLOCKS.glob("*/block.json")):
        build_archive(manifest.parent)
