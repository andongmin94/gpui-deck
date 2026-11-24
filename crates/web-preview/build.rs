use serde::Deserialize;
use std::{
    collections::BTreeSet,
    env,
    fmt::Write as _,
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Deserialize)]
struct Catalog {
    items: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    manifest: String,
}

#[derive(Deserialize)]
struct Item {
    name: String,
    #[serde(default)]
    recipes: Vec<String>,
    files: Vec<File>,
}

#[derive(Deserialize)]
struct File {
    source: String,
    target: String,
}

fn recipe_exports(names: &[String]) -> String {
    let mut exports = BTreeSet::new();
    let mut pending: Vec<_> = names.iter().map(String::as_str).collect();
    let mut seen = BTreeSet::new();
    while let Some(name) = pending.pop() {
        if seen.insert(name) {
            let recipe = gpui_deck::recipe_sources::RECIPES
                .iter()
                .find(|recipe| recipe.name == name)
                .unwrap_or_else(|| panic!("unknown registry recipe {name}"));
            exports.extend(recipe.exports.iter().copied());
            pending.extend(recipe.dependencies.iter().copied());
        }
    }
    exports
        .into_iter()
        .map(|name| format!("pub(crate) use crate::recipes::{name};\n"))
        .collect()
}

// Compile each preview with the files and recipe exports its manifest actually installs.
fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let installed = out.join("installed");
    if installed.exists() {
        fs::remove_dir_all(&installed).unwrap();
    }
    let mut modules = String::new();
    let mut names = String::new();
    let mut cases = String::new();
    for collection in ["neobrutal", "neumorphism"] {
        let registry = Path::new("../../registries").join(collection);
        println!("cargo:rerun-if-changed={}", registry.display());
        let catalog: Catalog =
            serde_json::from_slice(&fs::read(registry.join("registry.json")).unwrap()).unwrap();
        for entry in catalog.items {
            let path = registry.join(entry.manifest);
            // These original previews share the web form accessibility bridge.
            if collection == "neobrutal"
                && matches!(
                    path.file_stem().and_then(|s| s.to_str()),
                    Some("login" | "button" | "input")
                )
            {
                continue;
            }
            let item: Item = serde_json::from_slice(
                &fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display())),
            )
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let name = item.name;
            assert!(
                name.strip_prefix(collection)
                    .is_some_and(|suffix| suffix.starts_with('-'))
                    && name
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
                "invalid registry module {name}"
            );
            let module = name.replace('-', "_");
            let directory = installed.join(&module);
            for file in item.files {
                for relative in [&file.source, &file.target] {
                    assert!(
                        Path::new(relative)
                            .components()
                            .all(|part| matches!(part, Component::Normal(_))),
                        "invalid source mapping {relative}"
                    );
                }
                let source = registry.join(file.source);
                let target = directory.join(file.target);
                fs::create_dir_all(target.parent().unwrap()).unwrap();
                fs::copy(&source, target)
                    .unwrap_or_else(|error| panic!("{}: {error}", source.display()));
            }
            writeln!(
                modules,
                "mod {module} {{ use crate::{{Appearance, Palette}};
                mod recipes {{ {} }}
                #[path = {:?}] mod item;
                pub(super) use item::preview;
             }}",
                recipe_exports(&item.recipes),
                directory.join("mod.rs")
            )
            .unwrap();
            writeln!(names, "{name:?},").unwrap();
            writeln!(cases, "{name:?} => {module}::preview(window, cx),").unwrap();
        }
    }
    fs::write(out.join("registry.rs"), format!("{modules}
        pub(super) const NAMES: &[&str] = &[{names}];
        pub(super) fn open(name: &str, window: &mut gpui::Window, cx: &mut gpui::App) -> Option<gpui::AnyView> {{
            match name {{ {cases} _ => return None }}.into()
        }}")).unwrap();
}
