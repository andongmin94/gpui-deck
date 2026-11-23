use gpui::{
    AnyView, App, AppContext as _, Context, Div, Entity, IntoElement, ParentElement as _, Render,
    Role, Styled as _, Window, div, px,
};
use gpui_deck::InputState;

use crate::neobrutal::{Login, PAPER, editor, field, press_button};

pub(super) const NAMES: &[&str] = &["neobrutal-button", "neobrutal-input", "neobrutal-login"];

pub(super) fn open(name: &str, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    Some(match name {
        "neobrutal-button" => cx.new(|_| ButtonDemo { presses: 0 }).into(),
        "neobrutal-input" => cx
            .new(|cx| {
                let email = cx.new(|cx| editor(window, cx, "you@example.com", Role::EmailInput));
                cx.observe(&email, |_, _, cx| cx.notify()).detach();
                InputDemo { email }
            })
            .into(),
        "neobrutal-login" => cx.new(|cx| Login::new(window, cx)).into(),
        _ => return None,
    })
}

fn stage() -> Div {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.))
        .bg(gpui::rgb(PAPER))
}

pub(crate) struct ButtonDemo {
    presses: usize,
}

impl ButtonDemo {
    pub(crate) fn label(&self) -> String {
        match self.presses {
            0 => "Press me".into(),
            1 => "Pressed once".into(),
            n => format!("Pressed {n} times"),
        }
    }

    pub(crate) fn press(&mut self, cx: &mut Context<Self>) {
        self.presses += 1;
        cx.notify();
    }
}

impl Render for ButtonDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        stage().child(
            press_button("press", self.label())
                .on_click(cx.listener(|this, _, _, cx| this.press(cx))),
        )
    }
}

pub(crate) struct InputDemo {
    pub(crate) email: Entity<InputState>,
}

impl Render for InputDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        stage().child(div().w_full().max_w(px(300.)).child(field(
            "email",
            "Email address",
            &self.email,
            None,
            window,
            cx,
        )))
    }
}
