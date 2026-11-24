//! A toolbar with one tab stop and left/right, Home and End navigation.
mod button;
mod control_style;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use std::rc::Rc;

pub(crate) struct ToolbarState {
    tools: Vec<(SharedString, FocusHandle)>,
    active: usize,
}

impl ToolbarState {
    pub(crate) fn new(
        labels: impl IntoIterator<Item = impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tools: labels
                .into_iter()
                .map(|label| (label.into(), cx.focus_handle()))
                .collect(),
            active: 0,
        }
    }
}

pub(crate) fn toolbar(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<ToolbarState>,
    p: Palette,
    cx: &App,
    activate: impl Fn(usize, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let id = id.into();
    let model = state.read(cx);
    let navigation = state.clone();
    let activate = Rc::new(activate);
    div()
        .id(id.clone())
        .role(Role::Toolbar)
        .aria_label(label.into())
        .flex()
        .items_center()
        .gap_2()
        .p_2()
        .rounded(p.radius)
        .bg(p.surface)
        .shadow(palette::raised(p))
        .on_key_down(move |event, window, cx| {
            if !event.keystroke.modifiers.modified() {
                navigation.update(cx, |s, cx| {
                    let count = s.tools.len();
                    if count == 0 {
                        return;
                    }
                    let next = match event.keystroke.key.as_str() {
                        "left" => (s.active + count - 1) % count,
                        "right" => (s.active + 1) % count,
                        "home" => 0,
                        "end" => count - 1,
                        _ => return,
                    };
                    s.active = next;
                    s.tools[next].1.focus(window, cx);
                    cx.stop_propagation();
                    cx.notify();
                });
            }
        })
        .children(model.tools.iter().enumerate().map(|(i, (label, focus))| {
            let select = state.clone();
            let activate = activate.clone();
            button::press_button_with_palette((id.clone(), i.to_string()), label.clone(), p)
                .track_focus(focus)
                .tab_stop(i == model.active)
                .px_3()
                .on_click(move |_, window, cx| {
                    select.update(cx, |s, cx| {
                        s.active = i;
                        cx.notify();
                    });
                    activate(i, window, cx);
                })
        }))
}

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    let last = cx.new(|_| SharedString::from("Document tools"));
    let state = cx.new(|cx| ToolbarState::new(["Edit", "Review", "Publish"], cx));
    control_style::sample(state, cx, move |p, state, _, cx| {
        let output = last.clone();
        div()
            .flex()
            .flex_col()
            .gap_4()
            .items_center()
            .child(toolbar(
                "tools",
                "Document tools",
                state,
                p,
                cx,
                move |i, _, cx| {
                    output.update(cx, |text, cx| {
                        *text = ["Editing document", "Reviewing document", "Ready to publish"][i]
                            .into();
                        cx.notify();
                    });
                },
            ))
            .child(last.read(cx).clone())
    })
}
