use std::{fs, path::Path, time::SystemTime};

use gpui::{Modifiers, TestAppContext, VisualTestContext};
use gpui_deck::{Root, test_support};

use crate::screen::open_records;

fn open<'a>(path: &Path, app: &'a mut TestAppContext) -> &'a mut VisualTestContext {
    let (_, cx) = app.add_window_view(|w, cx| {
        let screen = open_records(path.into(), w, cx).unwrap();
        Root::new(screen, w, cx)
    });
    cx.update(|w, cx| {
        w.activate_window();
        w.draw(cx).clear(cx);
    });
    cx
}

fn click(cx: &mut VisualTestContext, label: &str) {
    let point = cx.update(|w, _| {
        test_support::snapshots(w)
            .iter()
            .find(|item| item.label() == Some(label) && item.visible())
            .unwrap_or_else(|| panic!("expected visible control {label}"))
            .bounds()
            .center()
    });
    cx.simulate_click(point, Modifiers::default());
    cx.update(|w, cx| w.draw(cx).clear(cx));
}

#[gpui::test]
fn mounted_host_saves_and_deletes_across_reopen(app: &mut TestAppContext) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("deck-host-{}-{timestamp}.json", std::process::id()));
    app.update(gpui_deck::init);
    let cx = open(&path, app);
    click(cx, "Edit Review team budget");
    click(cx, "Archived");
    click(cx, "Save record");
    let rows: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(rows[0]["id"], "TEAM-101");
    assert_eq!(rows[0]["status"], "Archived");
    cx.update(|w, _| w.remove_window());

    let cx = open(&path, app);
    click(cx, "Archived");
    click(cx, "Select Review team budget");
    click(cx, "Delete selected");
    click(cx, "Confirm deletion");
    cx.update(|w, _| w.remove_window());

    let cx = open(&path, app);
    cx.update(|w, _| {
        let controls = test_support::snapshots(w);
        assert!(
            controls
                .iter()
                .any(|item| item.label() == Some("Edit Schedule design review"))
        );
        assert!(
            !controls
                .iter()
                .any(|item| item.label() == Some("Edit Review team budget"))
        );
    });
    fs::remove_file(path).unwrap();
}
