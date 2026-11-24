//! Searchable selection with filtering, empty results, and keyboard interaction.

#[allow(dead_code)]
mod control_style;
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Combobox, ComboboxList, ComboboxOption, ComboboxState, SearchItem};
pub(crate) fn combobox(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<ComboboxState>,
    p: Palette,
    cx: &App,
) -> Combobox {
    let id = id.into();
    let label = label.into();
    let m = state.read(cx);
    let visible = m.visible_indices(cx);
    let list = ComboboxList::new((id.clone(), "options"), state)
        .accessibility_label(label.clone())
        .max_h(px(170.))
        .children(visible.iter().map(|&i| {
            let item = &m.items(cx)[i];
            ComboboxOption::new(state, i)
                .h(px(34.))
                .px_2()
                .flex()
                .items_center()
                .justify_between()
                .when(m.highlighted_index(cx) == Some(i), |s| {
                    s.bg(p.highlight).text_color(p.foreground)
                })
                .when(item.is_disabled(), |s| s.opacity(0.5))
                .child(item.label().clone())
                .when(m.is_selected(i), |s| s.child("✓"))
        }));
    Combobox::new(id, state)
        .input(move |input| {
            input
                .accessibility_label(label.clone())
                .placeholder(label)
                .w(px(240.))
                .h(px(40.))
                .px_3()
                .rounded(palette::control_radius(p))
                .border_1()
                .border_color(p.border)
                .bg(p.field)
                .shadow(palette::inset(p))
                .styles(|s| {
                    s.focused(|s| {
                        s.border_color(p.border)
                            .shadow(vec![gpui_deck::box_shadow(0., 0., 0., 2., p.accent)])
                    })
                })
        })
        .content(
            control_style::panel(p)
                .w(px(240.))
                .p_1()
                .child(list)
                .when(visible.is_empty(), |s| s.child("No matches")),
        )
}
pub(crate) fn preview(w: &mut Window, cx: &mut App) -> AnyView {
    let s = cx.new(|cx| {
        ComboboxState::new(
            [
                SearchItem::new("rust", "Rust"),
                SearchItem::new("zig", "Zig"),
                SearchItem::new("swift", "Swift"),
                SearchItem::new("go", "Go"),
            ],
            w,
            cx,
        )
    });
    let editor = s.read(cx).input().clone();
    editor.update(cx, |editor, _| style_editor(editor, palette::air()));
    control_style::sample(s, cx, |p, s, _, cx| {
        combobox("languages", "Find a language…", s, p, cx)
    })
}
