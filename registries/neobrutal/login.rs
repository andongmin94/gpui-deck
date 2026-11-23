//! A source-owned sign-in screen composed from the Neobrutal input and button.

mod button;
mod input;
mod palette;

pub(crate) use button::press_button;
pub(crate) use input::{editor, field};
pub(crate) use palette::PAPER;

use gpui::{
    App, AppContext as _, Context, Entity, Focusable as _, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, Role, StatefulInteractiveElement as _, Styled as _,
    Subscription, Window, div, prelude::FluentBuilder as _, px, rgb, white,
};
use gpui_deck::{InputState, box_shadow, input::InputEvent};
use palette::{ACCENT, INK, RADIUS};

pub(crate) struct Login {
    pub(crate) email: Entity<InputState>,
    pub(crate) password: Entity<InputState>,
    pub(crate) validated: bool,
    attempted: bool,
    _subscriptions: [Subscription; 2],
}

impl Login {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let email = cx.new(|cx| editor(window, cx, "you@example.com", Role::EmailInput));
        let password =
            cx.new(|cx| editor(window, cx, "Enter a sample password", Role::PasswordInput));
        let subscriptions = [&email, &password].map(|input| {
            cx.subscribe_in(
                input,
                window,
                |this: &mut Self, _, event, window, cx| match event {
                    InputEvent::PressEnter {
                        secondary: false,
                        shift: false,
                    } => this.submit(window, cx),
                    InputEvent::Change => {
                        this.validated = false;
                        cx.notify();
                    }
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                    _ => {}
                },
            )
        });
        Self {
            email,
            password,
            validated: false,
            attempted: false,
            _subscriptions: subscriptions,
        }
    }

    pub(crate) fn visible_errors(&self, cx: &App) -> [Option<&'static str>; 2] {
        if !self.attempted {
            return [None, None];
        }
        let email = self.email.read(cx).value();
        [
            (!valid_email(email.trim())).then_some("Enter a valid email address."),
            (self.password.read(cx).text().len() == 0).then_some("Enter a password."),
        ]
    }

    pub(crate) fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.attempted = true;
        let errors = self.visible_errors(cx);
        self.validated = errors == [None, None];
        if let Some((invalid, _)) = [&self.email, &self.password]
            .into_iter()
            .zip(errors)
            .find(|(_, error)| error.is_some())
        {
            invalid.focus_handle(cx).focus(window, cx);
        }
        // Connect a successful local validation to your authentication service here.
        cx.notify();
    }
}

fn valid_email(address: &str) -> bool {
    let Some((local, domain)) = address.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && local
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b".!#$%&'*+/=?^_`{|}~-".contains(&c))
        && domain.split('.').all(|label| {
            (1..=63).contains(&label.len())
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        })
}

impl Render for Login {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ink = rgb(INK);
        let [email_error, password_error] = self.visible_errors(cx);
        let heading = div()
            .flex()
            .flex_col()
            .gap_1()
            .px_6()
            .py_5()
            .rounded_t(px(RADIUS - 2.))
            .border_b_2()
            .border_color(ink)
            .bg(rgb(ACCENT))
            .child(
                div()
                    .id("neo-login-title")
                    .role(Role::Heading)
                    .aria_level(1)
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_size(px(36.))
                    .line_height(px(40.))
                    .child("Sign in"),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .child("Try a sample email and password."),
            );
        let body = div()
            .flex()
            .flex_col()
            .gap_5()
            .p_6()
            .child(
                field(
                    "neo-email",
                    "Email address",
                    &self.email,
                    email_error,
                    window,
                    cx,
                )
                .required(true),
            )
            .child(
                field(
                    "neo-password",
                    "Password",
                    &self.password,
                    password_error,
                    window,
                    cx,
                )
                .required(true),
            )
            .child(
                press_button("neo-login-submit", "Sign in")
                    .w_full()
                    .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
            )
            .when(self.validated, |body| {
                body.child(
                    div()
                        .id("neo-login-status")
                        .role(Role::Status)
                        .aria_label("Sample credentials validated locally.")
                        .font_weight(FontWeight::MEDIUM)
                        .text_size(px(14.))
                        .child("Sample credentials validated locally."),
                )
            });
        let card = div()
            .w_full()
            .max_w(px(414.))
            .rounded(px(RADIUS))
            .border_2()
            .border_color(ink)
            .bg(white())
            .shadow(vec![box_shadow(4., 4., 0., 0., ink.into())])
            .child(heading)
            .child(body);
        div()
            .id("neo-login-screen")
            .size_full()
            .overflow_y_scroll()
            .bg(rgb(PAPER))
            .text_color(ink)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p_5()
                    .child(card),
            )
    }
}
