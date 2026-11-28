//! An application-owned data source for the installed Neumorphism Data Manager.
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use gpui::{
    AnyView, App, AppContext as _, Context, IntoElement, Render, SharedString, Window, div,
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::ui::{
    Appearance,
    neumorphism_template_data_manager::{DataManager, Record, Status},
};

pub(crate) const TITLE: &str = "Team records";
pub(crate) const WINDOW_SIZE: (f32, f32) = (1100., 760.);

pub(crate) fn mount(window: &mut Window, cx: &mut App) -> AnyView {
    open_records("team-records.json".into(), window, cx).unwrap_or_else(|error| {
        cx.new(|_| LoadError(format!("Could not open team records: {error}").into()))
            .into()
    })
}

pub(crate) fn open_records(
    path: PathBuf,
    window: &mut Window,
    cx: &mut App,
) -> Result<AnyView, String> {
    let records = load(&path)?.into_iter().map(|row| row.0).collect();
    let save_path = path.clone();
    Ok(cx
        .new(|cx| {
            DataManager::new(Appearance::Settings.palette(), records, window, cx)
                .on_save(move |record| {
                    update(&save_path, |rows| {
                        if let Some(row) = rows.iter_mut().find(|row| row.0.id == record.id) {
                            row.0 = record.clone();
                        } else {
                            rows.push(TeamRecord(record.clone()));
                        }
                    })
                })
                .on_delete(move |ids| {
                    update(&path, |rows| rows.retain(|row| !ids.contains(&row.0.id)))
                })
        })
        .into())
}

// The host owns its JSON format without adding serialization to the installed block.
#[derive(Serialize, Deserialize)]
struct TeamRecord(#[serde(with = "RecordFields")] Record);

#[derive(Serialize, Deserialize)]
#[serde(remote = "Record")]
struct RecordFields {
    id: String,
    title: String,
    owner: String,
    #[serde(with = "StatusFields")]
    status: Status,
    date: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Status")]
enum StatusFields {
    Draft,
    Active,
    Archived,
}

fn load(path: &Path) -> Result<Vec<TeamRecord>, String> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|error| error.to_string()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(vec![
            TeamRecord(Record {
                id: "TEAM-101".into(),
                title: "Review team budget".into(),
                owner: "Jules".into(),
                status: Status::Active,
                date: String::new(),
            }),
            TeamRecord(Record {
                id: "TEAM-102".into(),
                title: "Schedule design review".into(),
                owner: "Rin".into(),
                status: Status::Draft,
                date: String::new(),
            }),
        ]),
        Err(error) => Err(error.to_string()),
    }
}

fn update(path: &Path, apply: impl FnOnce(&mut Vec<TeamRecord>)) -> Result<(), String> {
    let mut rows = load(path)?;
    apply(&mut rows);
    let json = serde_json::to_vec_pretty(&rows).map_err(|error| error.to_string())?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, json)
        .and_then(|()| fs::rename(temporary, path))
        .map_err(|error| format!("Could not write {}: {error}", path.display()))
}

struct LoadError(SharedString);

impl Render for LoadError {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_4().child(self.0.clone())
    }
}
