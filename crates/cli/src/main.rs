use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command as ProcessCommand, Stdio},
};

use anyhow::{Context as _, Result, bail};
use block::{Block, BlockIndex, InstalledBlock, Source};
use gpui_deck::recipe_sources::{LICENSE, PRESENTATION, RECIPES, Recipe};

mod block;
mod changes;
mod preset;

const USAGE: &str = "GPUI Deck source recipes

Usage:
  gpui-deck list
  gpui-deck add <recipe>... --to <module-directory>
  gpui-deck add <github-manifest-url> [--to <module-directory>]
  gpui-deck add --preset <code> [--to <module-directory>]
  gpui-deck diff --to <module-directory>

Example:
  gpui-deck add data-table avatar --to src/ui

Adds recipe dependencies, presentation tokens, module declarations and license.
GitHub block URLs also install the author's files. The default block destination
is src/ui. URL installs require curl on PATH. Create presets install from the
CLI bundle into src/ui/preset by default.
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
    AddBlock { url: String, to: PathBuf },
    AddPreset { code: String, to: PathBuf },
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
            let mut preset = None;
            while let Some(argument) = arguments.next() {
                if argument == "--to" {
                    if to.is_some() {
                        bail!("supply --to once");
                    }
                    to = Some(PathBuf::from(
                        arguments.next().context("--to needs a directory")?,
                    ));
                } else if argument == "--preset" {
                    if preset.is_some() {
                        bail!("supply --preset once");
                    }
                    let code = arguments.next().context("--preset needs a preset code")?;
                    let code = code
                        .into_string()
                        .map_err(|_| anyhow::anyhow!("preset code must be UTF-8"))?;
                    preset::validate(&code)?;
                    preset = Some(code);
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
            if let Some(code) = preset {
                if !names.is_empty() {
                    bail!("add one preset at a time, without recipe names or a block URL");
                }
                return Ok(Command::AddPreset {
                    code,
                    to: to.unwrap_or_else(|| "src/ui".into()),
                });
            }
            if names.is_empty() {
                bail!("add needs at least one recipe; use list to see available names");
            }
            if names.iter().any(|name| name.contains("://")) {
                if names.len() != 1 {
                    bail!("add one block URL at a time");
                }
                return Ok(Command::AddBlock {
                    url: names.pop().unwrap(),
                    to: to.unwrap_or_else(|| "src/ui".into()),
                });
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
        let declaration = format!("pub(crate) use {}::{exports};", recipe.module);
        if declaration.len() > 100 && recipe.exports.len() > 1 {
            source.push_str(&format!(
                "pub(crate) use {}::{{\n    {},\n}};\n",
                recipe.module,
                recipe.exports.join(", ")
            ));
        } else {
            source.push_str(&declaration);
            source.push('\n');
        }
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

fn matches_generated(path: &Path, contents: &[u8], expected: &str) -> bool {
    matches_text(contents, expected)
        || (path == Path::new("preset/selected.rs")
            && matches_text(contents, &expected.replace(" = \"", " =\n    \"")))
        || (path == Path::new("recipes/mod.rs")
            && matches_text(
                contents,
                &expected.replace("::{\n    ", "::{").replace(",\n};", "};"),
            ))
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

fn installed_blocks(to: &Path) -> Result<BlockIndex> {
    let Some(contents) = read(&to.join(block::INDEX))? else {
        return Ok(BlockIndex::new());
    };
    serde_json::from_slice(&contents)
        .with_context(|| format!("read {}", to.join(block::INDEX).display()))
}

fn generated_files(
    names: &BTreeSet<String>,
    blocks: &BlockIndex,
    include_preset_selection: bool,
) -> Result<BTreeMap<PathBuf, String>> {
    let mut root = ROOT_MODULE.to_owned();
    for module in blocks.keys() {
        block::check_module(module)?;
        root.push_str(&format!("pub(crate) mod {module};\n"));
    }
    let mut files = BTreeMap::from([
        ("mod.rs".into(), root),
        ("recipes/mod.rs".into(), module_source(names)?),
        (MANIFEST.into(), manifest_source(names)),
        ("LICENSE-APACHE".into(), LICENSE.to_owned()),
    ]);
    if !blocks.is_empty() {
        files.insert(
            block::INDEX.into(),
            serde_json::to_string_pretty(blocks)? + "\n",
        );
    }
    if include_preset_selection
        && let Some(entry) = blocks.get(preset::MODULE)
        && let Source::Preset(code) = &entry.source
    {
        preset::validate(code)?;
        files.insert(
            Path::new(preset::MODULE).join("selected.rs"),
            preset::selection(code),
        );
    }
    Ok(files)
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
    install(names, None, to)
}

fn check_destination(to: &Path, relative: &Path) -> Result<()> {
    let mut path = to.to_owned();
    let mut components = relative.components().peekable();
    while let Some(component) = components.next() {
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                #[cfg(windows)]
                let link =
                    std::os::windows::fs::MetadataExt::file_attributes(&metadata) & 0x400 != 0;
                #[cfg(not(windows))]
                let link = metadata.is_symlink();
                let valid_kind = if components.peek().is_some() {
                    metadata.is_dir()
                } else {
                    metadata.is_file()
                };
                if link || !valid_kind {
                    bail!(
                        "expected an ordinary installation path at {}",
                        path.display()
                    );
                }
            }
            Err(error) if error.kind() == ErrorKind::NotFound => break,
            Err(error) => return Err(error).with_context(|| format!("inspect {}", path.display())),
        }
    }
    Ok(())
}

fn install(mut names: Vec<String>, block: Option<Block>, to: &Path) -> Result<Installed> {
    // Resolve and preflight the complete addition before writing any source.
    let _lock = changes::lock(to)?;
    check_destination(to, Path::new(MANIFEST))?;
    check_destination(to, Path::new(block::INDEX))?;
    let installed = installed_recipes(to)?;
    let has_manifest = installed.is_some();
    let previous = installed.unwrap_or_default();
    let mut blocks = installed_blocks(to)?;
    let installing_preset = block
        .as_ref()
        .is_some_and(|item| matches!(item.source, Source::Preset(_)));
    let old = generated_files(&previous, &blocks, installing_preset)?;
    let mut sources = BTreeMap::new();
    if let Some(block) = block {
        let module = block.module;
        let alternate = to.join(format!("{module}.rs"));
        if alternate.try_exists()? {
            bail!("{} already declares this block module", alternate.display());
        }
        let entry = blocks
            .entry(module.clone())
            .or_insert_with(|| InstalledBlock {
                source: block.source.clone(),
                files: BTreeSet::new(),
            });
        if entry.source != block.source {
            if matches!(
                (&entry.source, &block.source),
                (Source::Preset(_), Source::Preset(_))
            ) {
                entry.source = block.source;
            } else {
                bail!("block {module} is already installed from another source");
            }
        }
        names.extend(block.recipes);
        for (file, bytes) in block.files {
            let path = Path::new(&module).join(&file);
            let previously_installed = !entry.files.insert(file);
            sources.insert(path, (bytes, previously_installed));
        }
    }
    let selected = resolve(previous.iter().cloned().chain(names))?;
    let previous_files = source_files(&previous)?;
    for (path, source) in source_files(&selected)? {
        let previously_installed = has_manifest && previous_files.contains_key(&path);
        sources.insert(path, (source.as_bytes().to_vec(), previously_installed));
    }
    let mut pending = BTreeMap::new();
    let mut result = Installed::default();
    for (path, (bytes, previously_installed)) in sources {
        check_destination(to, &path)?;
        match read(&to.join(&path))? {
            Some(contents)
                if previously_installed
                    || contents == bytes
                    || std::str::from_utf8(&bytes)
                        .is_ok_and(|text| matches_text(&contents, text)) =>
            {
                result.kept.push(path);
            }
            Some(_) => bail!(
                "{} already exists with different contents; choose a dedicated module directory",
                to.join(&path).display()
            ),
            None => {
                pending.insert(path, bytes);
            }
        }
    }
    for (path, new) in generated_files(&selected, &blocks, installing_preset)? {
        check_destination(to, &path)?;
        match read(&to.join(&path))? {
            Some(contents) if matches_generated(&path, &contents, &new) => result.kept.push(path),
            Some(contents)
                if old
                    .get(&path)
                    .is_some_and(|text| matches_generated(&path, &contents, text)) =>
            {
                pending.insert(path, new.into_bytes());
            }
            Some(_) => bail!(
                "{} was modified; preserve your changes and add the module declarations manually or choose another --to directory",
                to.join(&path).display()
            ),
            None => {
                pending.insert(path, new.into_bytes());
            }
        }
    }
    result.written = changes::write(to, pending)?;
    Ok(result)
}

fn diff(to: &Path, mut writer: impl Write) -> Result<()> {
    let _lock = changes::lock(to)?;
    let installed = installed_recipes(to)?
        .with_context(|| format!("no installed recipe manifest in {}", to.display()))?;
    let selected = resolve(installed)?;
    let blocks = installed_blocks(to)?;
    let mut files: BTreeMap<_, Vec<u8>> = generated_files(&selected, &blocks, true)?
        .into_iter()
        .map(|(path, source)| (path, source.into_bytes()))
        .collect();
    files.extend(
        source_files(&selected)?
            .into_iter()
            .map(|(path, source)| (path, source.as_bytes().to_owned())),
    );
    for (module, entry) in &blocks {
        if let Source::Preset(_) = entry.source {
            files.extend(
                preset::FILES
                    .iter()
                    .map(|(file, bytes)| (Path::new(module).join(file), bytes.to_vec())),
            );
        }
    }
    let mut different = false;
    for (path, bundled) in files {
        let local = read(&to.join(&path))?;
        if local.as_deref().is_some_and(|contents| {
            contents == bundled
                || std::str::from_utf8(&bundled)
                    .is_ok_and(|source| matches_generated(&path, contents, source))
        }) {
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
        child.stdin.take().unwrap().write_all(&bundled)?;
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
    if blocks
        .values()
        .any(|entry| matches!(entry.source, Source::Github(_)))
    {
        writeln!(
            writer,
            "GitHub block files are application-owned and were not compared."
        )?;
    }
    Ok(())
}

fn report(to: &Path, result: Installed) {
    for path in result.written {
        println!("added {}", to.join(path).display());
    }
    for path in result.kept {
        println!("kept {}", to.join(path).display());
    }
    println!("Declare this directory as a module in its parent (for example, mod ui;).");
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
            report(&to, add(names, &to)?);
            println!(
                "Import its recipes and Appearance::Settings.palette() or Appearance::Compact.palette()."
            );
        }
        Command::AddBlock { url, to } => {
            let block = block::load(&url, block::download)?;
            println!("Installing block module {}", block.module);
            report(&to, install(Vec::new(), Some(block), &to)?);
        }
        Command::AddPreset { code, to } => {
            report(&to, install(Vec::new(), Some(preset::block(code)), &to)?);
            println!("Use the installed preset::showcase(dark, window, cx) to open your design.");
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

    mod default_preset {
        include!("../presets/create/selected.rs");
    }

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
            parse(arguments(&["add", "--preset", default_preset::CODE])).unwrap(),
            Command::AddPreset {
                code: default_preset::CODE.into(),
                to: "src/ui".into()
            }
        );
        assert_eq!(
            parse(arguments(&[
                "add",
                "--to",
                "app/ui",
                "--preset",
                default_preset::CODE
            ]))
            .unwrap(),
            Command::AddPreset {
                code: default_preset::CODE.into(),
                to: "app/ui".into()
            }
        );
        assert_eq!(
            parse(arguments(&["add", EXAMPLE_URL])).unwrap(),
            Command::AddBlock {
                url: EXAMPLE_URL.into(),
                to: "src/ui".into()
            }
        );
        assert_eq!(
            parse(arguments(&["add", EXAMPLE_URL, "--to", "app/widgets"])).unwrap(),
            Command::AddBlock {
                url: EXAMPLE_URL.into(),
                to: "app/widgets".into()
            }
        );
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
            vec!["add", EXAMPLE_URL, "card"],
            vec!["add", "--preset"],
            vec!["add", "--preset", "v1.neumorphism"],
            vec![
                "add",
                "--preset",
                default_preset::CODE,
                "--preset",
                default_preset::CODE,
            ],
            vec!["add", "card", "--preset", default_preset::CODE],
            vec!["add", EXAMPLE_URL, "--preset", default_preset::CODE],
            vec!["diff"],
            vec!["diff", "--to"],
            vec!["diff", "card", "--to", "src/ui"],
            vec!["diff", "--to", "a", "--to", "b"],
        ] {
            assert!(parse(arguments(&invalid)).is_err());
        }
    }

    #[test]
    fn preset_owns_its_sources_and_replaces_only_an_unmodified_selection() {
        let directory = Directory::new();
        let to = &directory.0;
        let add_preset = |code: &str| install(Vec::new(), Some(preset::block(code.to_owned())), to);
        let first = default_preset::CODE;
        let next = "v1.neumorphism.sage.rose.forest.spectral.space-mono.solid.small";
        add_preset(first).unwrap();
        assert!(
            fs::read_to_string(to.join("mod.rs"))
                .unwrap()
                .contains("pub(crate) mod preset;")
        );
        let selected = to.join("preset/selected.rs");
        assert_eq!(
            fs::read_to_string(&selected).unwrap(),
            preset::selection(first)
        );
        let mut report = Vec::new();
        diff(to, &mut report).unwrap();
        assert_eq!(report, b"Installed files match the bundled sources.\n");

        let custom = to.join("preset/style.rs");
        fs::write(&custom, "// my styles").unwrap();
        assert!(add_preset(first).unwrap().written.is_empty());
        let long = "v1.neumorphism.lavender.neutral.neutral.pretendard.pretendard.outline.default";
        add_preset(long).unwrap();
        fs::write(
            &selected,
            preset::selection(long).replace(" = \"", " =\n    \""),
        )
        .unwrap();
        assert!(add_preset(long).unwrap().written.is_empty());
        add_preset(next).unwrap();
        assert_eq!(
            fs::read_to_string(&selected).unwrap(),
            preset::selection(next)
        );
        assert_eq!(fs::read_to_string(custom).unwrap(), "// my styles");
        fs::write(&selected, "// my preset").unwrap();
        add(vec!["card".into()], to).unwrap();
        let old_index = fs::read(to.join(block::INDEX)).unwrap();
        assert!(add_preset(first).is_err());
        assert_eq!(fs::read_to_string(selected).unwrap(), "// my preset");
        assert_eq!(fs::read(to.join(block::INDEX)).unwrap(), old_index);
        report.clear();
        diff(to, &mut report).unwrap();
        assert!(String::from_utf8(report).unwrap().contains("-// my styles"));
    }

    #[test]
    fn malformed_preset_and_existing_block_module_leave_the_destination_untouched() {
        let directory = Directory::new();
        let to = &directory.0;
        let invalid = default_preset::CODE.replace(".outline.", ".unknown.");
        assert!(install(Vec::new(), Some(preset::block(invalid)), to).is_err());
        assert_eq!(fs::read_dir(to).unwrap().count(), 0);
        let existing =
            sample(EXAMPLE_URL, &EXAMPLE.replace("billing-dashboard", "preset")).unwrap();
        install(Vec::new(), Some(existing), to).unwrap();
        assert!(
            install(
                Vec::new(),
                Some(preset::block(default_preset::CODE.into())),
                to
            )
            .is_err()
        );
        assert_eq!(
            fs::read_to_string(to.join("preset/mod.rs")).unwrap(),
            "mod palette;"
        );
        assert!(!to.join("preset/selected.rs").exists());
    }

    const EXAMPLE_URL: &str = "https://github.com/acme/ui/blob/main/billing/block.json";
    const RAW_URL: &str = "https://raw.githubusercontent.com/acme/ui/main/billing/block.json";
    const EXAMPLE: &str = r#"{
        "version": 1, "name": "billing-dashboard", "recipes": ["chart", "data-table"],
        "files": [{"source":"screen.rs","target":"mod.rs"},{"source":"brand.rs","target":"palette.rs"},
                  {"source":"logo.png","target":"assets/logo.png"}]
    }"#;

    fn sample(url: &str, manifest: &str) -> Result<Block> {
        block::load(url, |url| match url.rsplit('/').next().unwrap() {
            "block.json" => Ok(manifest.as_bytes().to_vec()),
            "screen.rs" => Ok(b"mod palette;".to_vec()),
            "brand.rs" => Ok(b"// team palette".to_vec()),
            "logo.png" => Ok(vec![0, 255, 10]),
            other => bail!("fixture not found: {other}"),
        })
    }

    #[test]
    fn github_block_and_bundled_recipes_share_an_editable_installation() {
        let directory = Directory::new();
        let to = &directory.0;
        let block = sample(EXAMPLE_URL, EXAMPLE).unwrap();
        assert_eq!(block.source, Source::Github(RAW_URL.into()));
        assert_eq!(
            sample(&RAW_URL.replace("/main/", "/refs/heads/main/"), EXAMPLE)
                .unwrap()
                .source,
            Source::Github(RAW_URL.into())
        );
        install(Vec::new(), Some(block), to).unwrap();
        let palette = to.join("billing_dashboard/palette.rs");
        assert_eq!(fs::read_to_string(&palette).unwrap(), "// team palette");
        assert_eq!(
            fs::read(to.join("billing_dashboard/assets/logo.png")).unwrap(),
            [0, 255, 10]
        );
        assert_eq!(
            fs::read_to_string(to.join(MANIFEST)).unwrap(),
            "chart\ndata-table\nempty\ninput-group\npagination\n"
        );
        assert!(
            fs::read_to_string(to.join("mod.rs"))
                .unwrap()
                .contains("pub(crate) mod billing_dashboard;")
        );
        fs::write(&palette, "// app palette").unwrap();
        add(vec!["card".into()], to).unwrap();
        let same = install(Vec::new(), Some(sample(RAW_URL, EXAMPLE).unwrap()), to).unwrap();
        assert!(same.written.is_empty());
        assert_eq!(fs::read_to_string(&palette).unwrap(), "// app palette");
        let mut output = Vec::new();
        diff(to, &mut output).unwrap();
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("GitHub block files")
        );
        let other = sample(&EXAMPLE_URL.replace("/acme/", "/other/"), EXAMPLE).unwrap();
        assert!(install(Vec::new(), Some(other), to).is_err());
    }

    #[test]
    fn block_and_recipe_conflicts_abort_the_entire_install() {
        for conflict in [
            "billing_dashboard/palette.rs",
            "recipes/chart.rs",
            "billing_dashboard.rs",
        ] {
            let directory = Directory::new();
            let destination = directory.0.join(conflict);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::write(&destination, "// existing app source").unwrap();
            assert!(
                install(
                    Vec::new(),
                    Some(sample(EXAMPLE_URL, EXAMPLE).unwrap()),
                    &directory.0
                )
                .is_err()
            );
            assert_eq!(
                fs::read_to_string(destination).unwrap(),
                "// existing app source"
            );
            assert!(!directory.0.join("mod.rs").exists());
            assert!(!directory.0.join("billing_dashboard/mod.rs").exists());
            assert!(!directory.0.join(MANIFEST).exists());
        }
    }

    #[test]
    fn malformed_blocks_and_unavailable_sources_fail_before_install() {
        for invalid in [
            EXAMPLE.replace("\"version\": 1", "\"version\": 2"),
            EXAMPLE.replace("billing-dashboard", "presentation"),
            EXAMPLE.replace("billing-dashboard", "mod"),
            EXAMPLE.replace("screen.rs", "../screen.rs"),
            EXAMPLE.replace("palette.rs", "MOD.RS"),
            EXAMPLE.replace("palette.rs", "/palette.rs"),
            EXAMPLE.replace("palette.rs", "mod.rs/child"),
            EXAMPLE.replace("brand.rs", "missing.rs"),
            EXAMPLE.replace("logo.png", ".git/config"),
        ] {
            assert!(sample(EXAMPLE_URL, &invalid).is_err());
        }
        assert!(sample("https://example.com/blocks/billing.json", EXAMPLE).is_err());
    }

    #[test]
    fn read_only_declarations_preserve_the_existing_installation_for_retry() {
        let directory = Directory::new();
        let to = &directory.0;
        add(vec!["card".into()], to).unwrap();
        let modules = to.join("recipes/mod.rs");
        let original = modules.metadata().unwrap().permissions();
        let mut read_only = original.clone();
        read_only.set_readonly(true);
        fs::set_permissions(&modules, read_only).unwrap();
        let result = install(Vec::new(), Some(sample(EXAMPLE_URL, EXAMPLE).unwrap()), to);
        fs::set_permissions(&modules, original).unwrap();
        assert!(result.is_err());
        assert_eq!(fs::read_to_string(to.join(MANIFEST)).unwrap(), "card\n");
        assert!(!to.join(block::INDEX).exists());
        assert!(!to.join("billing_dashboard/mod.rs").exists());
        install(Vec::new(), Some(sample(EXAMPLE_URL, EXAMPLE).unwrap()), to).unwrap();
    }

    #[test]
    fn simultaneous_additions_preserve_both_manifests_and_legacy_export_format() {
        let directory = Directory::new();
        let to = directory.0.join("ui");
        std::thread::scope(|scope| {
            scope.spawn(|| add(vec!["button".into()], &to).unwrap());
            scope.spawn(|| {
                install(Vec::new(), Some(sample(EXAMPLE_URL, EXAMPLE).unwrap()), &to).unwrap()
            });
        });
        let exported = fs::read_to_string(to.join("recipes/mod.rs")).unwrap();
        assert!(exported.contains("pub(crate) use button::"));
        assert!(exported.contains("pub(crate) use data_table::{\n"));
        let legacy = exported.replace("::{\n    ", "::{").replace(",\n};", "};");
        fs::write(to.join("recipes/mod.rs"), legacy).unwrap();
        add(vec!["badge".into()], &to).unwrap();
        assert!(to.join("billing_dashboard/mod.rs").exists());
        assert!(
            fs::read_to_string(to.join("recipes/mod.rs"))
                .unwrap()
                .contains("pub(crate) use data_table::{\n")
        );
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
