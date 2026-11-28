use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, Result, bail, ensure};
use gpui_deck::recipe_sources::{PRETENDARD_MEDIUM, PRETENDARD_SEMIBOLD};

use crate::{changes, install, preset};

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    directory: PathBuf,
    code: String,
    dark: bool,
}

impl Project {
    pub fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Self> {
        let mut directory = None;
        let mut code = None;
        let mut dark = false;
        while let Some(argument) = args.next() {
            match argument.to_str() {
                Some("--preset") => {
                    ensure!(code.is_none(), "supply --preset once");
                    let value = args
                        .next()
                        .context("--preset needs a preset code or Create URL")?;
                    code = Some(preset::parse(
                        value.to_str().context("preset must be UTF-8")?,
                    )?);
                }
                Some("--dark") => {
                    ensure!(!dark, "supply --dark once");
                    dark = true;
                }
                Some(flag) if flag.starts_with('-') => bail!("unknown option {flag}"),
                _ => {
                    ensure!(directory.is_none(), "init takes one new project directory");
                    directory = Some(PathBuf::from(argument));
                }
            }
        }
        Ok(Self {
            directory: directory.context("init needs a new project directory")?,
            code: code.unwrap_or_else(|| preset::default::CODE.to_owned()),
            dark,
        })
    }

    pub fn create(&self) -> Result<&Path> {
        let destination = std::path::absolute(&self.directory)?;
        let name = destination
            .file_name()
            .context("expected a new package directory")?;
        let parent = destination
            .parent()
            .context("expected a parent directory")?;
        let _lock = changes::lock(&destination)?;
        match fs::symlink_metadata(&destination) {
            Ok(_) => bail!(
                "{} already exists; choose a new directory",
                destination.display()
            ),
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        let stage = tempfile::Builder::new()
            .prefix(".gpui-deck-init-")
            .tempdir_in(parent)
            .with_context(|| format!("stage the project in {}", parent.display()))?;
        // Let Cargo validate the package name and create a binary crate. Keep
        // its automatic workspace enrollment inside the staging directory.
        fs::write(
            stage.path().join("Cargo.toml"),
            "[workspace]\nresolver = \"3\"\n",
        )?;
        let result = Command::new("cargo")
            .current_dir(stage.path())
            .args([
                "new",
                "--quiet",
                "--bin",
                "--edition",
                "2024",
                "--vcs",
                "none",
                "--name",
            ])
            .arg(name)
            .arg("app")
            .output()
            .context("run cargo new (Cargo must be on PATH)")?;
        ensure!(
            result.status.success(),
            "cargo new: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        );
        let app = stage.path().join("app");
        let src = app.join("src");
        let dependencies = include_str!("../templates/dependencies.toml")
            .replace("GPUI_DECK_VERSION", env!("CARGO_PKG_VERSION"));
        writeln!(
            OpenOptions::new()
                .append(true)
                .open(app.join("Cargo.toml"))?,
            "\n{dependencies}"
        )?;
        fs::write(app.join(".gitignore"), "/target\n")?;
        install(
            Vec::new(),
            Some(preset::block(self.code.clone())),
            &src.join("ui"),
        )?;
        fs::write(
            src.join("gallery.rs"),
            include_bytes!("../templates/gallery.rs"),
        )?;
        fs::write(
            src.join("main.rs"),
            include_str!("../templates/main.rs")
                .replace("CREATE_DARK_MODE", if self.dark { "true" } else { "false" }),
        )?;
        let fonts = src.join("fonts");
        fs::create_dir(&fonts)?;
        fs::write(fonts.join("Pretendard-Medium.otf"), PRETENDARD_MEDIUM)?;
        fs::write(fonts.join("Pretendard-SemiBold.otf"), PRETENDARD_SEMIBOLD)?;
        fs::rename(app, &destination)
            .with_context(|| format!("create {}", destination.display()))?;
        Ok(&self.directory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command as CliCommand, add, diff, parse};

    #[test]
    fn create_url_and_appearance_initialize_an_editable_standalone_project() {
        let parent = tempfile::Builder::new()
            .prefix("deck workspace ")
            .tempdir()
            .unwrap();
        let workspace = "[workspace]\nmembers = []\n";
        fs::write(parent.path().join("Cargo.toml"), workspace).unwrap();
        let app = parent.path().join("team-app");
        let selected = "v1.neumorphism.sage.rose.forest.spectral.space-mono.solid.small";
        let CliCommand::Init(project) = parse([
            "init".into(),
            app.clone().into(),
            "--preset".into(),
            format!("https://gpui-deck.andongmin.com/ko/create?preset={selected}").into(),
            "--dark".into(),
        ])
        .unwrap() else {
            panic!("expected init")
        };
        project.create().unwrap();
        assert_eq!(
            fs::read_to_string(parent.path().join("Cargo.toml")).unwrap(),
            workspace
        );
        let manifest = fs::read_to_string(app.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("name = \"team-app\""));
        assert!(manifest.contains(&format!("version = \"={}\"", env!("CARGO_PKG_VERSION"))));
        assert!(
            fs::read_to_string(app.join("src/main.rs"))
                .unwrap()
                .contains("gallery::open(true,")
        );
        assert_eq!(
            fs::read(app.join("src/gallery.rs")).unwrap(),
            include_bytes!("../templates/gallery.rs")
        );
        let ui = app.join("src/ui");
        assert_eq!(
            fs::read_to_string(ui.join("preset/selected.rs")).unwrap(),
            preset::selection(selected)
        );
        add(vec!["button".to_owned()], &ui).unwrap();
        diff(&ui, Vec::new()).unwrap();
        fs::write(app.join("src/main.rs"), "// user's application").unwrap();
        assert!(project.create().is_err());
        assert_eq!(
            fs::read_to_string(app.join("src/main.rs")).unwrap(),
            "// user's application"
        );
    }

    #[test]
    fn defaults_and_invalid_names_or_presets_leave_the_destination_intact() {
        let parent = tempfile::tempdir().unwrap();
        let path = parent.path().join("bad package name");
        let project = Project::parse([path.clone().into()].into_iter()).unwrap();
        assert_eq!(project.code, preset::default::CODE);
        assert!(!project.dark);
        assert!(project.create().is_err());
        assert_eq!(fs::read_dir(parent.path()).unwrap().count(), 0);
        for args in [
            vec!["--dark"],
            vec!["app", "two"],
            vec!["app", "--dark", "--dark"],
            vec!["app", "--preset"],
            vec!["app", "--preset", "--dark"],
            vec!["app", "--preset", "https://example.com/create"],
        ] {
            assert!(Project::parse(args.into_iter().map(OsString::from)).is_err());
        }
    }
}
