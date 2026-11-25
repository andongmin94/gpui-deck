use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    process::Command,
};

use anyhow::{Context as _, Result, bail, ensure};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    name: String,
    #[serde(default)]
    recipes: Vec<String>,
    files: Vec<SourceFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceFile {
    source: String,
    target: String,
}

pub struct Block {
    pub module: String,
    pub source: Source,
    pub recipes: Vec<String>,
    pub files: BTreeMap<PathBuf, Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Source {
    #[serde(rename = "url")]
    Github(String),
    #[serde(rename = "preset")]
    Preset(String),
}

#[derive(Deserialize, Serialize)]
pub struct InstalledBlock {
    #[serde(flatten)]
    pub source: Source,
    pub files: BTreeSet<PathBuf>,
}

pub type BlockIndex = BTreeMap<String, InstalledBlock>;
pub const INDEX: &str = ".gpui-deck-blocks.json";

fn raw_urls(url: &str) -> Result<(String, String)> {
    let url = url.split(['?', '#']).next().unwrap();
    let (path, blob) = if let Some(path) = url.strip_prefix("https://github.com/") {
        (path, true)
    } else if let Some(path) = url.strip_prefix("https://raw.githubusercontent.com/") {
        (path, false)
    } else {
        bail!(
            "use an HTTPS GitHub file URL or raw.githubusercontent.com URL for the block manifest"
        );
    };
    let mut parts: Vec<_> = path.split('/').collect();
    if blob {
        ensure!(
            parts.get(2) == Some(&"blob"),
            "expected a GitHub file URL containing /blob/"
        );
        parts.remove(2);
    }
    ensure!(
        parts.len() >= 4
            && parts.iter().all(|part| !part.is_empty()
                && *part != "."
                && *part != ".."
                && !part.contains(['\\', '\r', '\n'])),
        "expected owner/repository/revision/path in the GitHub URL"
    );
    let raw = |parts: &[&str]| format!("https://raw.githubusercontent.com/{}", parts.join("/"));
    let fetch = raw(&parts);
    // GitHub's "Raw" link includes refs/heads/. Download the exact ref but
    // store one identity for both links to the same branch.
    if parts.len() >= 6 && parts[2..4] == ["refs", "heads"] {
        parts.drain(2..4);
    }
    Ok((fetch, raw(&parts)))
}

// A portable Rust module name and portable relative file paths keep an author's
// manifest usable on all three supported native platforms.
fn module_name(name: &str) -> Result<String> {
    ensure!(
        name.starts_with(|c: char| c.is_ascii_lowercase())
            && name.split('-').all(|part| !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())),
        "block name must be lowercase kebab-case"
    );
    let module = name.replace('-', "_");
    const RESERVED: &str = concat!(
        "assets presentation recipes as async await break const continue crate ",
        "dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return ",
        "self static struct super trait true type unsafe use where while abstract become box do ",
        "final gen macro override priv try typeof unsized virtual yield"
    );
    ensure!(
        !RESERVED.split_ascii_whitespace().any(|word| word == module),
        "block name {name} is reserved"
    );
    relative_path(&module)?;
    Ok(module)
}

pub fn check_module(module: &str) -> Result<()> {
    ensure!(
        module_name(&module.replace('_', "-"))? == module,
        "invalid block module {module}"
    );
    Ok(())
}

fn relative_path(path: &str) -> Result<()> {
    for part in path.split('/') {
        let stem = part.split('.').next().unwrap().to_ascii_uppercase();
        let device = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
            || stem
                .strip_prefix("COM")
                .or_else(|| stem.strip_prefix("LPT"))
                .is_some_and(|suffix| {
                    suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9')
                });
        ensure!(
            part.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                && !part.ends_with('.')
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
                && !device,
            "invalid relative block path {path:?}"
        );
    }
    Ok(())
}

pub fn download(url: &str) -> Result<Vec<u8>> {
    // --disable must be first. Local curl aliases/config must not rewrite an
    // install, interpret glob syntax, or save a server response to another file.
    let output = Command::new("curl")
        .args([
            "--disable",
            "--fail",
            "--silent",
            "--show-error",
            "--globoff",
            "--proto",
            "=https",
            "--",
            url,
        ])
        .output()
        .context("download GitHub block (curl must be on PATH)")?;
    ensure!(
        output.status.success(),
        "download {url}: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(output.stdout)
}

pub fn load(url: &str, fetch: impl Fn(&str) -> Result<Vec<u8>>) -> Result<Block> {
    let (url, canonical) = raw_urls(url)?;
    let manifest: Manifest = serde_json::from_slice(&fetch(&url)?)
        .with_context(|| format!("read block manifest {url}"))?;
    ensure!(
        manifest.version == 1,
        "unsupported block manifest version {}",
        manifest.version
    );
    let module = module_name(&manifest.name)?;
    let mut targets = BTreeSet::new();
    for file in &manifest.files {
        relative_path(&file.source)?;
        relative_path(&file.target)?;
        ensure!(
            targets.insert(PathBuf::from(file.target.to_ascii_lowercase())),
            "duplicate target {}",
            file.target
        );
    }
    ensure!(
        manifest.files.iter().any(|file| file.target == "mod.rs"),
        "a block must provide mod.rs"
    );
    for target in &targets {
        ensure!(
            !target
                .ancestors()
                .skip(1)
                .any(|parent| targets.contains(parent)),
            "block file/directory conflict at {}",
            target.display()
        );
    }
    let directory = url.rsplit_once('/').unwrap().0;
    let mut files = BTreeMap::new();
    for file in manifest.files {
        let contents = fetch(&format!("{directory}/{}", file.source))?;
        files.insert(PathBuf::from(file.target), contents);
    }
    Ok(Block {
        module,
        source: Source::Github(canonical),
        recipes: manifest.recipes,
        files,
    })
}
