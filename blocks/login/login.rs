//! A local sign-in form. Connect the successful submit branch to your authentication service.

mod palette;

use gpui::{
    App, AppContext as _, Context, Entity, Focusable as _, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, Role, StatefulInteractiveElement as _, Styled as _,
    Subscription, Window, div, prelude::FluentBuilder as _, px,
};
use gpui_deck::{
    Button, Field, Input, InputEditorStyle, InputState,
    input::{InputAccessibility, InputEvent},
};

use super::{
    Palette,
    recipes::{ButtonVariant, InputGroup},
};
use palette::login_palette;

pub(crate) struct Login {
    email: Entity<InputState>,
    password: Entity<InputState>,
    attempted: bool,
    validated: bool,
    _events: [Subscription; 2],
}

impl Login {
    pub(crate) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let palette = login_palette();
        let [email, password] = [
            (Role::EmailInput, "name@example.com", false),
            (Role::PasswordInput, "Enter a sample password", true),
        ]
        .map(|(role, placeholder, secret)| {
            cx.new(|cx| {
                let mut input = InputState::new(window, cx)
                    .placeholder(placeholder)
                    .masked(secret);
                input.set_accessibility(
                    InputAccessibility::new(Some(role)).with_value_exposed(!secret),
                );
                input.set_editor_style(InputEditorStyle {
                    foreground: palette.foreground,
                    muted_foreground: palette.muted,
                    selection: palette.highlight,
                    caret: palette.accent,
                    ..Default::default()
                });
                input
            })
        });
        let events = [&email, &password].map(|input| {
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
            attempted: false,
            validated: false,
            _events: events,
        }
    }

    fn errors(&self, cx: &App) -> [Option<&'static str>; 2] {
        let value = self.email.read(cx).value();
        let email = value.trim();
        let valid_email = email.split_once('@').is_some_and(|(local, host)| {
            !local.is_empty() && !host.is_empty() && !host.contains('@')
        }) && !email.contains(char::is_whitespace);
        [
            (!valid_email).then_some("Enter a valid email address."),
            (self.password.read(cx).text().len() == 0).then_some("Enter a password."),
        ]
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.attempted = true;
        let errors = self.errors(cx);
        self.validated = errors == [None, None];
        if let Some((input, _)) = [&self.email, &self.password]
            .into_iter()
            .zip(errors)
            .find(|(_, error)| error.is_some())
        {
            input.focus_handle(cx).focus(window, cx);
        }
        // The preview only validates the form. The consumer supplies authentication here.
        cx.notify();
    }
}

impl Render for Login {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = login_palette();
        let errors = if self.attempted {
            self.errors(cx)
        } else {
            [None, None]
        };
        let masked = self.password.read(cx).presentation().is_masked();
        let reveal_label = if masked {
            "Show password"
        } else {
            "Hide password"
        };
        let reveal = ButtonVariant::Ghost
            .apply(Button::new("login-reveal"), palette)
            .h(px(30.))
            .px_2()
            .text_size(px(13.))
            .accessibility_label(reveal_label)
            .child(if masked { "Show" } else { "Hide" })
            .on_click(cx.listener(|this, _, window, cx| {
                this.password
                    .update(cx, |input, cx| input.toggle_masked(window, cx));
                cx.notify();
            }));
        let email = InputGroup::new("login-email-input", Input::new(&self.email), palette)
            .h(palette.control_height)
            .focused(self.email.focus_handle(cx).is_focused(window))
            .invalid(errors[0].is_some());
        let password = InputGroup::new("login-password-input", Input::new(&self.password), palette)
            .h(palette.control_height)
            .focused(self.password.focus_handle(cx).is_focused(window))
            .invalid(errors[1].is_some())
            .trailing(reveal);
        let fields = [
            ("login-email", "Email address", &self.email, email),
            ("login-password", "Password", &self.password, password),
        ]
        .into_iter()
        .zip(errors)
        .map(|((id, label, input, control), error)| {
            Field::new(id, input, label)
                .required(true)
                .error(error.map(Into::into))
                .styles(|styles| {
                    styles
                        .label(|s| s.font_weight(FontWeight::MEDIUM).text_size(px(14.)))
                        .error(|s| s.text_color(palette.error).text_size(px(13.)))
                })
                .flex()
                .flex_col()
                .gap(px(7.))
                .child(control)
        });
        div()
            .id("login-screen")
            .size_full()
            .overflow_y_scroll()
            .bg(palette.canvas)
            .text_color(palette.foreground)
            .text_size(palette.font_size)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p_4()
                    .child(
                        div()
                            .w_full()
                            .max_w(px(416.))
                            .flex()
                            .flex_col()
                            .gap_5()
                            .p_6()
                            .border_1()
                            .border_color(palette.border)
                            .rounded(px(16.))
                            .bg(palette.surface)
                            .child(lock_mark(palette))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .id("login-title")
                                            .role(Role::Heading)
                                            .aria_level(1)
                                            .text_size(px(30.))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Sign in"),
                                    )
                                    .child(
                                        div().text_color(palette.muted).child(
                                            "Try the form with a sample email and password.",
                                        ),
                                    ),
                            )
                            .children(fields)
                            .child(
                                ButtonVariant::Primary
                                    .apply(Button::new("login-submit"), palette)
                                    .w_full()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .accessibility_label("Sign in")
                                    .child("Sign in")
                                    .on_click(
                                        cx.listener(|this, _, window, cx| this.submit(window, cx)),
                                    ),
                            )
                            .when(self.validated, |card| {
                                card.child(
                                    div()
                                        .id("login-status")
                                        .role(Role::Status)
                                        .aria_label("Sample credentials validated locally.")
                                        .text_size(px(14.))
                                        .text_color(palette.accent)
                                        .child("Sample credentials validated locally."),
                                )
                            }),
                    ),
            )
    }
}

fn lock_mark(palette: Palette) -> impl IntoElement {
    div()
        .relative()
        .size(px(40.))
        .rounded(px(11.))
        .bg(palette.highlight)
        .child(
            div()
                .absolute()
                .top(px(8.))
                .left(px(14.))
                .w(px(12.))
                .h(px(14.))
                .rounded(px(6.))
                .border_2()
                .border_color(palette.accent),
        )
        .child(
            div()
                .absolute()
                .top(px(18.))
                .left(px(11.))
                .w(px(18.))
                .h(px(13.))
                .rounded(px(3.))
                .bg(palette.accent),
        )
}
