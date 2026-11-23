//! form in the application's Neobrutal palette.

#[allow(dead_code)]
mod button;
#[allow(dead_code)]
mod control_style;
#[allow(dead_code)]
mod palette;

#[allow(dead_code)]
mod editor_style;
pub(crate) use editor_style::style_editor;

use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{
    Field, Input, InputState,
    input::{InputEvent, InputMode},
};
/// Use the app's validation result for both visible error text and native metadata.
pub(crate) fn form_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: &Entity<InputState>,
    error: Option<SharedString>,
    p: Palette,
    window: &Window,
    cx: &App,
) -> Field<InputMode> {
    let frame = editor_style::frame(state, error.is_some(), p, window, cx);
    Field::new(id, state, label)
        .error(error)
        .styles(|s| {
            s.label(|s| s.font_weight(FontWeight::BOLD))
                .error(|s| s.text_color(p.error))
        })
        .flex()
        .flex_col()
        .gap_2()
        .child(frame.child(Input::new(state)))
}
struct WorkspaceForm {
    input: Entity<InputState>,
    submitted: bool,
    _subscription: Subscription,
}
impl Render for WorkspaceForm {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = palette::yellow();
        let missing = self.input.read(cx).value().trim().is_empty();
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .p_4()
            .bg(p.canvas)
            .text_color(p.foreground)
            .child(
                control_style::panel(p)
                    .w_full()
                    .max_w(px(320.))
                    .p_4()
                    .gap_3()
                    .child(
                        form_field(
                            "name",
                            "Workspace name",
                            &self.input,
                            (self.submitted && missing).then(|| "Enter a workspace name.".into()),
                            p,
                            window,
                            cx,
                        )
                        .required(true),
                    )
                    .child(
                        button::raised(p, gpui_deck::Button::new("save"))
                            .accessibility_label("Save")
                            .child("Save")
                            .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
                    )
                    .when(self.submitted && !missing, |s| {
                        s.child("Workspace name saved.")
                    }),
            )
    }
}
impl WorkspaceForm {
    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.submitted = true;
        if self.input.read(cx).value().trim().is_empty() {
            self.input.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let input = cx.new(|cx| {
            let mut input = InputState::new(window, cx).placeholder("Studio");
            style_editor(&mut input, palette::yellow());
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this: &mut WorkspaceForm, _, event, window, cx| match event {
                InputEvent::PressEnter {
                    secondary: false,
                    shift: false,
                } => this.submit(window, cx),
                InputEvent::Change => {
                    this.submitted = false;
                    cx.notify();
                }
                InputEvent::Focus | InputEvent::Blur => cx.notify(),
                _ => {}
            },
        );
        WorkspaceForm {
            input,
            submitted: false,
            _subscription: subscription,
        }
    })
    .into()
}
