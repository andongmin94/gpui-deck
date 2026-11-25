use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{Result, ensure};
use serde::Deserialize;

use crate::block::{Block, Source};

pub const MODULE: &str = "preset";
const AXES: [&str; 8] = [
    "style", "base", "accent", "chart", "heading", "font", "icons", "radius",
];

#[derive(Deserialize)]
struct Choice<'a> {
    value: &'a str,
}

pub fn validate(code: &str) -> Result<()> {
    let options: BTreeMap<&str, Vec<Choice<'_>>> =
        serde_json::from_str(include_str!("../presets/create/options.json"))?;
    let mut parts = code.split('.');
    ensure!(parts.next() == Some("v1"), "expected a v1 Create preset");
    for axis in AXES {
        let value = parts.next().unwrap_or_default();
        ensure!(
            options[axis].iter().any(|choice| choice.value == value),
            "invalid {axis} option {value:?} in Create preset"
        );
    }
    ensure!(
        parts.next().is_none(),
        "unexpected extra options in Create preset"
    );
    Ok(())
}

pub fn selection(code: &str) -> String {
    format!("pub const CODE: &str = \"{code}\";\n")
}

macro_rules! bundled {
    ($($path:literal),+ $(,)?) => {
        &[$(($path, include_bytes!(concat!("../presets/create/", $path)))),+]
    };
}

pub const FILES: &[(&str, &[u8])] = bundled![
    "mod.rs",
    "style.rs",
    "showcase.rs",
    "icons.rs",
    "assets/Neumorphism-LICENSE.txt",
    "assets/Pretendard-Regular.otf",
    "assets/Pretendard-LICENSE.txt",
    "assets/Spectral-Regular.ttf",
    "assets/Spectral-LICENSE.txt",
    "assets/SpaceMono-Regular.ttf",
    "assets/SpaceMono-LICENSE.txt",
    "assets/lucide-activity.svg",
    "assets/lucide-bookmark.svg",
    "assets/Lucide-LICENSE.txt",
    "assets/bootstrap-activity.svg",
    "assets/bootstrap-bookmark.svg",
    "assets/Bootstrap-LICENSE.txt",
];

pub fn block(code: String) -> Block {
    Block {
        module: MODULE.to_owned(),
        source: Source::Preset(code),
        recipes: Vec::new(),
        files: FILES
            .iter()
            .map(|(file, bytes)| (PathBuf::from(file), bytes.to_vec()))
            .collect(),
    }
}
