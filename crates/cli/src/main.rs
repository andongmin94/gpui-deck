use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command as ProcessCommand, Stdio},
};

use anyhow::{Context as _, Result, bail};
use gpui_deck::recipe_sources::{LICENSE, PRESENTATION, RECIPES, Recipe};

const USAGE: &str = "GPUI Deck source recipes

Usage:
  gpui-deck list
  gpui-deck add <recipe>... --to <module-directory>
  gpui-deck diff --to <module-directory>

Example:
  gpui-deck add data-table avatar --to src/ui

Adds recipe dependencies, presentation tokens, module declarations and license.
Existing installed recipe and token edits are kept. Modified generated module
declarations or conflicting new files stop the command before any files change.
The consumer supplies its gpui and gpui-deck Cargo dependencies.
Use diff to review installed files against the sources bundled with this CLI.
It prints changes using Git and leaves the installation untouched.";

const ROOT_MODULE: &str = "//! Application-owned GPUI Deck presentation sources.

mod presentation;
pub(crate) mod recipes;

pub(crate) use presentation::{Appearance, Palette};
";
const MANIFEST: &str = "recipes/.gpui-deck";

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    List,
    Add { names: Vec<String>, to: PathBuf },
    Diff { to: PathBuf },
}

fn parse(arguments: impl IntoIterator<Item = OsString>) -> Result<Command> {
    let mut arguments = arguments.into_iter();
    match arguments.next().as_deref().and_then(|value| value.to_str()) {
        None | Some("--help" | "-h") => Ok(Command::Help),
        Some("list") => {
            if arguments.next().is_some() {
                bail!("list does not accept arguments");
            }
            Ok(Command::List)
        }
        Some("diff") => {
            if arguments.next().as_deref() != Some("--to".as_ref()) {
                bail!("diff requires --to <module-directory>");
            }
            let to = arguments.next().context("--to needs a directory")?;
            if arguments.next().is_some() {
                bail!("diff takes one --to <module-directory>");
            }
            Ok(Command::Diff { to: to.into() })
        }
        Some("add") => {
            let mut names = Vec::new();
            let mut to = None;
            while let Some(argument) = arguments.next() {
                if argument == "--to" {
                    if to.is_some() {
                        bail!("supply --to once");
                    }
                    to = Some(PathBuf::from(
                        arguments.next().context("--to needs a directory")?,
                    ));
                } else {
                    let name = argument
                        .into_string()
                        .map_err(|_| anyhow::anyhow!("recipe names must be UTF-8"))?;
                    if name.starts_with('-') {
                        bail!("unknown option {name}");
                    }
                    names.push(name);
                }
            }
            if names.is_empty() {
                bail!("add needs at least one recipe; use list to see available names");
            }
            Ok(Command::Add {
                names,
                to: to.context("add requires --to <module-directory>")?,
            })
        }
        Some(command) => bail!("unknown command {command}"),
    }
}

fn recipe(name: &str) -> Result<&'static Recipe> {
    RECIPES
        .iter()
        .find(|recipe| recipe.name == name)
        .with_context(|| format!("unknown recipe {name}; use list to see available names"))
}

fn resolve(names: impl IntoIterator<Item = String>) -> Result<BTreeSet<String>> {
    let mut pending: Vec<_> = names.into_iter().collect();
    let mut selected = BTreeSet::new();
    while let Some(name) = pending.pop() {
        let item = recipe(&name)?;
        if selected.insert(name) {
            pending.extend(item.dependencies.iter().map(|name| (*name).to_owned()));
        }
    }
    Ok(selected)
}

fn module_source(names: &BTreeSet<String>) -> Result<String> {
    let mut source =
        "//! Recipe modules managed by gpui-deck add; edit the recipe files to customize them.\n\n"
            .to_owned();
    for name in names {
        let recipe = recipe(name)?;
        source.push_str(&format!("mod {};\n", recipe.module));
    }
    source.push('\n');
    for name in names {
        let recipe = recipe(name)?;
        let exports = if recipe.exports.len() == 1 {
            recipe.exports[0].to_owned()
        } else {
            format!("{{{}}}", recipe.exports.join(", "))
        };
        source.push_str(&format!("pub(crate) use {}::{exports};\n", recipe.module));
    }
    Ok(source)
}

fn manifest_source(names: &BTreeSet<String>) -> String {
    names.iter().map(|name| format!("{name}\n")).collect()
}

fn read(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::read(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("read {}", path.display())),
    }
}

// A consumer's Git checkout can change text line endings. Keep the on-disk
// bytes when the file still has the same contents.
fn matches_text(contents: &[u8], expected: &str) -> bool {
    contents == expected.as_bytes()
        || std::str::from_utf8(contents)
            .is_ok_and(|text| text.replace("\r\n", "\n") == expected.replace("\r\n", "\n"))
}

fn installed_recipes(to: &Path) -> Result<Option<BTreeSet<String>>> {
    let path = to.join(MANIFEST);
    let Some(contents) = read(&path)? else {
        return Ok(None);
    };
    let names = std::str::from_utf8(&contents)
        .context("installed recipe metadata must be UTF-8")?
        .lines()
        .map(str::to_owned)
        .collect();
    if !matches_text(&contents, &manifest_source(&names)) {
        bail!(
            "{} was modified; keep its recipe names in generated order",
            path.display()
        );
    }
    Ok(Some(names))
}

fn generated_files(names: &BTreeSet<String>) -> Result<BTreeMap<PathBuf, String>> {
    Ok(BTreeMap::from([
        ("mod.rs".into(), ROOT_MODULE.to_owned()),
        ("recipes/mod.rs".into(), module_source(names)?),
        (MANIFEST.into(), manifest_source(names)),
        ("LICENSE-APACHE".into(), LICENSE.to_owned()),
    ]))
}

fn source_files(names: &BTreeSet<String>) -> Result<BTreeMap<PathBuf, &'static str>> {
    let mut files = BTreeMap::from([(PathBuf::from("presentation.rs"), PRESENTATION)]);
    for name in names {
        let recipe = recipe(name)?;
        files.insert(
            PathBuf::from(format!("recipes/{}.rs", recipe.module)),
            recipe.source,
        );
        for (path, source) in recipe.assets {
            files.insert(PathBuf::from(path), source);
        }
    }
    Ok(files)
}

#[derive(Default)]
struct Installed {
    written: Vec<PathBuf>,
    kept: Vec<PathBuf>,
}

fn add(names: Vec<String>, to: &Path) -> Result<Installed> {
    // Resolve and preflight the complete addition before writing any source.
    let installed = installed_recipes(to)?;
    let has_manifest = installed.is_some();
    let previous = installed.unwrap_or_default();
    let selected = resolve(previous.iter().cloned().chain(names))?;
    let previous_files = source_files(&previous)?;
    let mut pending = BTreeMap::new();
    let mut result = Installed::default();
    for (path, source) in source_files(&selected)? {
        match read(&to.join(&path))? {
            Some(_) if has_manifest && previous_files.contains_key(&path) => {
                result.kept.push(path);
            }
            Some(contents) if matches_text(&contents, source) => result.kept.push(path),
            Some(_) => bail!(
                "{} already exists with different contents; choose a dedicated module directory",
                to.join(&path).display()
            ),
            None => {
                pending.insert(path, source.to_owned());
            }
        }
    }
    let old = generated_files(&previous)?;
    for (path, new) in generated_files(&selected)? {
        match read(&to.join(&path))? {
            Some(contents) if matches_text(&contents, &new) => result.kept.push(path),
            Some(contents) if matches_text(&contents, &old[&path]) => {
                pending.insert(path, new);
            }
            Some(_) => bail!(
                "{} was modified; preserve your changes and add the module declarations manually or choose another --to directory",
                to.join(&path).display()
            ),
            None => {
                pending.insert(path, new);
            }
        }
    }
    for (path, source) in pending {
        let destination = to.join(&path);
        fs::create_dir_all(destination.parent().unwrap())
            .with_context(|| format!("create directory for {}", destination.display()))?;
        fs::write(&destination, source)
            .with_context(|| format!("write {}", destination.display()))?;
        result.written.push(path);
    }
    Ok(result)
}

fn diff(to: &Path, mut writer: impl Write) -> Result<()> {
    let installed = installed_recipes(to)?
        .with_context(|| format!("no installed recipe manifest in {}", to.display()))?;
    let selected = resolve(installed)?;
    let mut files = generated_files(&selected)?;
    files.extend(
        source_files(&selected)?
            .into_iter()
            .map(|(path, source)| (path, source.to_owned())),
    );
    let mut different = false;
    for (path, bundled) in files {
        let local = read(&to.join(&path))?;
        if local
            .as_deref()
            .is_some_and(|contents| matches_text(contents, &bundled))
        {
            continue;
        }
        // Git accepts '-' as stdin in --no-index mode, so the bundled version
        // never has to be written to the installation or to a temporary file.
        let old_path = if local.is_some() {
            path.as_path()
        } else {
            Path::new("/dev/null")
        };
        let mut child = ProcessCommand::new("git")
            .current_dir(to)
            .args([
                "-c",
                "core.autocrlf=false",
                "diff",
                "--no-index",
                "--no-ext-diff",
                "--no-textconv",
                "--no-color",
                "--ignore-cr-at-eol",
                "--",
            ])
            .arg(old_path)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("run git diff (Git must be on PATH)")?;
        child.stdin.take().unwrap().write_all(bundled.as_bytes())?;
        let output = child.wait_with_output()?;
        match output.status.code() {
            Some(0) => continue,
            Some(1) => {
                writeln!(writer, "{} (installed -> bundled)", path.display())?;
                writer.write_all(&output.stdout)?;
                different = true;
            }
            _ => bail!(
                "git diff for {}: {}",
                path.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        }
    }
    if !different {
        writeln!(writer, "Installed files match the bundled sources.")?;
    }
    Ok(())
}

fn run() -> Result<()> {
    match parse(std::env::args_os().skip(1))? {
        Command::Help => println!("{USAGE}"),
        Command::List => {
            for recipe in RECIPES {
                if recipe.dependencies.is_empty() {
                    println!("{}", recipe.name);
                } else {
                    println!(
                        "{} (includes {})",
                        recipe.name,
                        recipe.dependencies.join(", ")
                    );
                }
            }
        }
        Command::Diff { to } => diff(&to, io::stdout().lock())?,
        Command::Add { names, to } => {
            let result = add(names, &to)?;
            for path in result.written {
                println!("added {}", to.join(path).display());
            }
            for path in result.kept {
                println!("kept {}", to.join(path).display());
            }
            println!("Declare this directory as a module in its parent (for example, mod ui;).");
            println!(
                "Import its recipes and Appearance::Settings.palette() or Appearance::Compact.palette()."
            );
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}\n\n{USAGE}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Directory(PathBuf);
    impl Directory {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let root = std::env::temp_dir().canonicalize().unwrap();
            let path = root.join(format!(
                "gpui-deck-cli-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Directory {
        fn drop(&mut self) {
            let root = std::env::temp_dir().canonicalize().unwrap();
            let resolved = self.0.canonicalize().unwrap();
            assert!(resolved.starts_with(root) && resolved == self.0);
            fs::remove_dir_all(resolved).unwrap();
        }
    }

    #[test]
    fn arguments_require_named_recipes_and_one_destination() {
        let arguments = |items: &[&str]| items.iter().map(OsString::from).collect::<Vec<_>>();
        assert_eq!(parse(arguments(&["list"])).unwrap(), Command::List);
        assert_eq!(
            parse(arguments(&["diff", "--to", "src/ui"])).unwrap(),
            Command::Diff {
                to: "src/ui".into()
            }
        );
        assert_eq!(
            parse(arguments(&["add", "card", "badge", "--to", "src/ui"])).unwrap(),
            Command::Add {
                names: vec!["card".into(), "badge".into()],
                to: "src/ui".into()
            }
        );
        for invalid in [
            vec!["list", "card"],
            vec!["add", "card"],
            vec!["add", "--to", "src/ui"],
            vec!["add", "card", "--to"],
            vec!["add", "card", "--to", "a", "--to", "b"],
            vec!["diff"],
            vec!["diff", "--to"],
            vec!["diff", "card", "--to", "src/ui"],
            vec!["diff", "--to", "a", "--to", "b"],
        ] {
            assert!(parse(arguments(&invalid)).is_err());
        }
    }

    #[test]
    fn addition_includes_transitive_dependencies_and_exact_original_sources() {
        let directory = Directory::new();
        add(vec!["data-table".into(), "avatar".into()], &directory.0).unwrap();
        assert_eq!(
            fs::read_to_string(directory.0.join(MANIFEST)).unwrap(),
            "avatar\ndata-table\nempty\ninput-group\npagination\n"
        );
        assert_eq!(
            fs::read_to_string(directory.0.join("presentation.rs")).unwrap(),
            PRESENTATION
        );
        assert_eq!(
            fs::read_to_string(directory.0.join("recipes/data_table.rs")).unwrap(),
            recipe("data-table").unwrap().source
        );
        assert_eq!(
            fs::read_to_string(directory.0.join("LICENSE-APACHE")).unwrap(),
            LICENSE
        );
        assert!(directory.0.join("assets/preset-avatar.svg").is_file());
        let modules = fs::read_to_string(directory.0.join("recipes/mod.rs")).unwrap();
        assert!(modules.contains("mod pagination;"));
        assert!(!modules.contains("mod button_group;"));
        assert!(modules.contains("DataTableState"));
    }

    #[test]
    fn repeat_additions_keep_consumer_source_edits_and_merge_new_modules() {
        let directory = Directory::new();
        add(vec!["card".into()], &directory.0).unwrap();
        fs::write(directory.0.join("presentation.rs"), "// my palette\n").unwrap();
        fs::write(directory.0.join("recipes/card.rs"), "// my card\n").unwrap();
        add(vec!["message".into()], &directory.0).unwrap();
        let repeated = add(vec!["card".into()], &directory.0).unwrap();
        assert!(repeated.written.is_empty());
        assert_eq!(
            fs::read_to_string(directory.0.join("presentation.rs")).unwrap(),
            "// my palette\n"
        );
        assert_eq!(
            fs::read_to_string(directory.0.join("recipes/card.rs")).unwrap(),
            "// my card\n"
        );
        assert_eq!(
            fs::read_to_string(directory.0.join(MANIFEST)).unwrap(),
            "card\nmessage\ntypography\n"
        );
    }

    #[test]
    fn checked_out_crlf_files_can_be_compared_and_extended() {
        let directory = Directory::new();
        let to = &directory.0;
        add(vec!["card".into()], to).unwrap();
        let paths = [
            MANIFEST,
            "mod.rs",
            "recipes/mod.rs",
            "LICENSE-APACHE",
            "presentation.rs",
            "recipes/card.rs",
        ];
        let mut checkout = BTreeMap::new();
        for path in paths {
            let text = fs::read_to_string(to.join(path))
                .unwrap()
                .replace("\r\n", "\n")
                .replace('\n', "\r\n");
            fs::write(to.join(path), &text).unwrap();
            checkout.insert(path, text);
        }
        let mut output = Vec::new();
        diff(to, &mut output).unwrap();
        assert_eq!(output, b"Installed files match the bundled sources.\n");
        assert!(add(vec!["card".into()], to).unwrap().written.is_empty());
        for path in paths {
            assert_eq!(fs::read_to_string(to.join(path)).unwrap(), checkout[path]);
        }
        let badge = recipe("badge").unwrap().source;
        let existing_badge = if badge.contains("\r\n") {
            badge.replace("\r\n", "\n")
        } else {
            badge.replace('\n', "\r\n")
        };
        let badge_path = to.join("recipes/badge.rs");
        fs::write(&badge_path, &existing_badge).unwrap();
        add(vec!["badge".into()], to).unwrap();
        assert_eq!(fs::read_to_string(badge_path).unwrap(), existing_badge);
        for path in [
            "mod.rs",
            "LICENSE-APACHE",
            "presentation.rs",
            "recipes/card.rs",
        ] {
            assert_eq!(fs::read_to_string(to.join(path)).unwrap(), checkout[path]);
        }
        fs::write(to.join(MANIFEST), "badge\r\ncard\r\n\r\n").unwrap();
        assert!(diff(to, &mut output).is_err());
    }

    #[test]
    fn diff_shows_installed_edits_and_missing_files_without_writing() {
        let directory = Directory::new();
        let to = &directory.0;
        let mut output = Vec::new();
        assert!(diff(to, &mut output).is_err());
        assert_eq!(fs::read_dir(to).unwrap().count(), 0);

        add(vec!["avatar".into()], to).unwrap();
        diff(to, &mut output).unwrap();
        assert_eq!(output, b"Installed files match the bundled sources.\n");
        output.clear();
        fs::write(to.join("recipes/avatar.rs"), "// custom avatar\n").unwrap();
        fs::write(to.join("mod.rs"), "// custom root\n").unwrap();
        fs::remove_file(to.join("assets/preset-avatar.svg")).unwrap();
        fs::write(to.join("unrelated.txt"), "app content\n").unwrap();
        diff(to, &mut output).unwrap();

        let report = String::from_utf8(output).unwrap();
        assert!(report.contains("-// custom avatar"));
        assert!(report.contains("-// custom root"));
        assert!(report.contains("+pub(crate) mod recipes;"));
        assert!(report.contains("preset-avatar.svg (installed -> bundled)"));
        assert!(!report.contains("unrelated.txt"));
        assert_eq!(
            fs::read(to.join("recipes/avatar.rs")).unwrap(),
            b"// custom avatar\n"
        );
        assert_eq!(fs::read(to.join("mod.rs")).unwrap(), b"// custom root\n");
        assert!(!to.join("assets/preset-avatar.svg").exists());
    }

    #[test]
    fn conflicts_fail_before_writing_any_additional_files() {
        let directory = Directory::new();
        add(vec!["card".into()], &directory.0).unwrap();
        fs::write(directory.0.join("recipes/mod.rs"), "// custom module\n").unwrap();
        assert!(add(vec!["data-table".into()], &directory.0).is_err());
        assert!(!directory.0.join("recipes/data_table.rs").exists());
        assert_eq!(
            fs::read_to_string(directory.0.join("recipes/mod.rs")).unwrap(),
            "// custom module\n"
        );

        let fresh = Directory::new();
        fs::write(fresh.0.join("presentation.rs"), "// preexisting tokens\n").unwrap();
        assert!(add(vec!["card".into()], &fresh.0).is_err());
        assert!(!fresh.0.join("mod.rs").exists());
        assert!(!fresh.0.join("recipes/card.rs").exists());
    }

    #[test]
    fn unknown_recipes_do_not_create_an_installation() {
        let directory = Directory::new();
        assert!(add(vec!["unknown".into()], &directory.0).is_err());
        assert_eq!(fs::read_dir(&directory.0).unwrap().count(), 0);
        for item in RECIPES {
            resolve([item.name.to_owned()]).unwrap();
        }
    }
}
