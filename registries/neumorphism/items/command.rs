//! Search, navigate and activate commands from a filtered list.

#[allow(dead_code)]
mod control_style;
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    Command, CommandEvent, CommandInput, CommandList, CommandOption, CommandState, SearchItem,
};
pub(crate) fn command(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<CommandState>,
    p: Palette,
    cx: &App,
) -> Command {
    let id = id.into();
    let m = state.read(cx);
    let options = m.visible_indices().iter().map(|&i| {
        let item = &m.items()[i];
        CommandOption::new(state, i)
            .py_1()
            .px_2()
            .rounded(p.radius)
            .when(item.is_disabled(), |s| s.opacity(0.5))
            .when(m.highlighted_index() == Some(i), |s| {
                s.bg(p.highlight).text_color(p.foreground)
            })
            .child(item.label().clone())
    });
    Command::new(id.clone(), state)
        .accessibility_label(label)
        .w_full()
        .max_w(px(340.))
        .rounded(p.radius)
        .shadow(palette::raised(p))
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .flex()
        .flex_col()
        .child(
            div().p_3().border_b_1().border_color(p.border).child(
                CommandInput::new((id.clone(), "search"), state)
                    .accessibility_label("Search commands")
                    .placeholder("Search commands…"),
            ),
        )
        .child(
            CommandList::new((id, "results"), state)
                .accessibility_label("Results")
                .max_h(px(200.))
                .p_2()
                .flex()
                .flex_col()
                .children(options),
        )
        .when(m.visible_indices().is_empty(), |s| {
            s.child(div().p_2().child("No matching commands"))
        })
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        CommandState::new(
            [
                SearchItem::new("new", "New document"),
                SearchItem::new("open", "Open project"),
                SearchItem::new("theme", "Change theme"),
            ],
            w,
            cx,
        )
    });
    let last = cx.new(|_| SharedString::from("Choose a command"));
    let target = last.clone();
    cx.subscribe(&s, move |_, e: &CommandEvent, cx| {
        let CommandEvent::Action(value) = e;
        target.update(cx, |s, cx| {
            *s = format!("Executed: {value}").into();
            cx.notify();
        });
    })
    .detach();
    let editor = s.read(cx).input().clone();
    editor.update(cx, |editor, _| style_editor(editor, palette::air()));
    control_style::sample(s, cx, move |p, s, _, cx| {
        div()
            .w_full()
            .max_w(px(340.))
            .flex()
            .flex_col()
            .gap_2()
            .child(command("commands", "Project commands", s, p, cx))
            .child(last.read(cx).clone())
    })
}
