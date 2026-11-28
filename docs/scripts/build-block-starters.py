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
TEMPLATES = ROOT / "crates/cli/templates"


def stage_ui(ui, path, manifest):
    block = path.parent
    module = manifest["name"].replace("-", "_")
    block_dir = ui / module
    for file in manifest["files"]:
        source = (block / file["source"]).resolve()
        source.relative_to(block)
        target = (block_dir / file["target"]).resolve()
        target.relative_to(block_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)

    # Keep the same source identity as an install from the public GitHub URL.
    index = {
        module: {
            "url": f"https://raw.githubusercontent.com/andongmin94/gpui-deck/main/{path.relative_to(ROOT).as_posix()}",
            "files": sorted(file["target"] for file in manifest["files"]),
        }
    }
    (ui / ".gpui-deck-blocks.json").write_text(
        json.dumps(index, indent=2) + "\n", encoding="utf-8"
    )
    if not manifest.get("recipes"):
        # A registry with its own components has no bundled recipes to pass to
        # the CLI. Use its empty module templates and editable palette source.
        recipes = ui / "recipes"
        recipes.mkdir()
        (recipes / ".gpui-deck").write_bytes(b"")
        header = (TEMPLATES / "recipes-header.rs").read_text(encoding="utf-8").rstrip()
        (recipes / "mod.rs").write_text(header + "\n\n\n", encoding="utf-8")
        root = (TEMPLATES / "ui-root.rs").read_text(encoding="utf-8")
        (ui / "mod.rs").write_text(root + f"pub(crate) mod {module};\n", encoding="utf-8")
        shutil.copyfile(ROOT / "crates/base/examples/presentation.rs", ui / "presentation.rs")
        shutil.copyfile(ROOT / "crates/base/LICENSE-APACHE", ui / "LICENSE-APACHE")
        return
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


def registry_screen(module, slug, registry):
    title = slug.removeprefix("template-").removesuffix("-template").replace("-", " ").title()
    title = {"Cms": "CMS", "Analytics": "Analytics Dashboard"}.get(title, title)
    if module == "neobrutal_login":
        mount = """use gpui::{AnyView, App, AppContext as _, Window};

pub(crate) fn mount(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| crate::ui::neobrutal_login::Login::new(window, cx))
        .into()
}"""
        size = "(760., 640.)"
    else:
        mount = f"pub(crate) use crate::ui::{module}::preview as mount;"
        size = "(1100., 760.)"
    return (
        f'{mount}\n\npub(crate) const TITLE: &str = "{registry} · {title}";\n'
        f"pub(crate) const WINDOW_SIZE: (f32, f32) = {size};\n"
    )


def stage_app(app, path, manifest, registry):
    stage_ui(app / "src/ui", path, manifest)
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]
    dependencies = (TEMPLATES / "dependencies.toml").read_text(encoding="utf-8")
    package = f'[package]\nname = "{app.name}"\nversion = "0.1.0"\nedition = "2024"\n\n'
    (app / "Cargo.toml").write_text(
        package + dependencies.replace("GPUI_DECK_VERSION", workspace["package"]["version"]),
        encoding="utf-8",
    )
    for name in ("main.rs", "block_window.rs"):
        shutil.copyfile(BLOCKS / "starter" / name, app / "src" / name)
    screen = app / "src/screen.rs"
    if registry:
        screen.write_text(
            registry_screen(app.name.replace("-", "_"), path.stem, registry), encoding="utf-8"
        )
    else:
        shutil.copyfile(path.parent / "app/screen.rs", screen)
    fonts = app / "assets/fonts"
    fonts.mkdir(parents=True)
    for name in (
        "Pretendard-Regular.otf", "Pretendard-Medium.otf", "Pretendard-SemiBold.otf", "LICENSE"
    ):
        shutil.copyfile(ROOT / "crates/base/examples/assets/fonts" / name, fonts / name)
    (app / ".gitignore").write_text("/target\n", encoding="utf-8")


def build_archive(path, registry=None):
    manifest = json.loads(path.read_text(encoding="utf-8"))
    name = f"{path.parent.name}-{path.stem}" if registry else path.parent.name
    if manifest["version"] != 1 or manifest["name"] != name:
        raise ValueError(f"expected the {name} v1 block manifest at {path}")
    output = DOWNLOADS / f"{name}.zip"
    with tempfile.TemporaryDirectory(dir=DOWNLOADS) as staging:
        stage = Path(staging)
        app = stage / name
        stage_app(app, path, manifest, registry)
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
        build_archive(manifest)
    for path in sorted((ROOT / "registries").glob("*/registry.json")):
        catalog = json.loads(path.read_text(encoding="utf-8"))
        for item in catalog["items"]:
            if item["kind"] in {"block", "template"}:
                build_archive(path.parent / item["manifest"], catalog["name"])
