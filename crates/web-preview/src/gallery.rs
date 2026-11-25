//! Optional Create example. Place this file next to the installed `preset` module.
use super::preset::{self, Preset, Showcase, Tokens};
use gpui::{prelude::*, *};
use gpui_deck::{Checkbox, CheckboxState, Progress, Switch};

pub struct Gallery {
    sample: Entity<Showcase>,
    tokens: Tokens,
    checklist: [bool; 4],
    notifications: [bool; 2],
}

pub fn open(dark: bool, window: &mut Window, cx: &mut App) -> AnyView {
    preset::register_fonts(cx);
    let preset = preset::selected();
    let sample = cx.new(|cx| Showcase::new(preset, dark, window, cx));
    cx.new(|_| Gallery {
        sample,
        tokens: preset.tokens(dark),
        checklist: [true, true, false, false],
        notifications: [true, false],
    })
    .into()
}

impl Gallery {
    /// Apply a new selection without replacing the editable reports or checklist.
    pub fn set_preset(&mut self, preset: Preset, dark: bool, cx: &mut Context<Self>) {
        self.tokens = preset.tokens(dark);
        self.sample
            .update(cx, |s, cx| s.set_preset(preset, dark, cx));
        cx.notify();
    }

    fn card(&self, title: &'static str) -> Div {
        let t = self.tokens;
        t.panel()
            .w_full()
            .min_w_0()
            .p_5()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .font_family(t.heading)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(px(21.))
                    .child(title),
            )
    }

    fn checklist(&self, cx: &Context<Self>) -> Div {
        let t = self.tokens;
        let count = self.checklist.iter().filter(|done| **done).count();
        let fraction = count as f32 / self.checklist.len() as f32;
        self.card("Release checklist")
            .child(div().text_color(t.muted).child(format!(
                "{count} of {} tasks complete",
                self.checklist.len()
            )))
            .child(
                Progress::new("readiness")
                    .accessibility_label("Release readiness")
                    .value(fraction * 100.)
                    .w_full()
                    .h(px(7.))
                    .rounded_full()
                    .overflow_hidden()
                    .bg(t.field)
                    .child(div().h_full().w(relative(fraction)).bg(t.accent)),
            )
            .children(
                [
                    "Review changes",
                    "Run test suite",
                    "Write release notes",
                    "Publish release",
                ]
                .into_iter()
                .enumerate()
                .map(|(i, label)| {
                    let done = self.checklist[i];
                    let gallery = cx.entity();
                    Checkbox::new(label)
                        .accessibility_label(label)
                        .state(if done {
                            CheckboxState::Checked
                        } else {
                            CheckboxState::Unchecked
                        })
                        .flex()
                        .items_center()
                        .gap_2()
                        .min_h(px(30.))
                        .cursor_pointer()
                        .focus_visible(|s| s.underline())
                        .child(
                            div()
                                .size(px(17.))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.))
                                .border_1()
                                .border_color(if done { t.accent } else { t.muted })
                                .when(done, |v| v.bg(t.accent).text_color(t.on_accent).child("✓")),
                        )
                        .child(label)
                        .on_change(move |state, _, _, cx| {
                            gallery.update(cx, |this, cx| {
                                this.checklist[i] = state == CheckboxState::Checked;
                                cx.notify();
                            })
                        })
                }),
            )
    }

    fn notifications(&self, cx: &Context<Self>) -> Div {
        let t = self.tokens;
        self.card("Notifications").children(
            ["Release announcements", "Weekly digest"]
                .into_iter()
                .enumerate()
                .map(|(i, label)| {
                    let checked = self.notifications[i];
                    let gallery = cx.entity();
                    Switch::new(label)
                        .accessibility_label(label)
                        .checked(checked)
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .py_2()
                        .cursor_pointer()
                        .focus_visible(|s| s.underline())
                        .child(label)
                        .child(
                            div()
                                .w(px(34.))
                                .h(px(20.))
                                .p(px(2.))
                                .flex()
                                .flex_shrink_0()
                                .rounded_full()
                                .bg(if checked { t.accent } else { t.muted })
                                .when(checked, |v| v.justify_end())
                                .child(div().size(px(16.)).rounded_full().bg(if checked {
                                    t.on_accent
                                } else {
                                    t.surface
                                })),
                        )
                        .on_change(move |value, _, _, cx| {
                            gallery.update(cx, |this, cx| {
                                this.notifications[i] = value;
                                cx.notify();
                            })
                        })
                }),
        )
    }

    fn history(&self) -> Div {
        let t = self.tokens;
        self.card("Release history").children(
            [
                ("2.4.0", "Stable", "Nov 24"),
                ("2.4.0-rc.1", "Preview", "Nov 18"),
                ("2.3.2", "Stable", "Nov 10"),
            ]
            .map(|(version, channel, date)| {
                div()
                    .flex()
                    .gap_3()
                    .items_center()
                    .border_t_1()
                    .border_color(t.border)
                    .pt_3()
                    .child(
                        div()
                            .flex_1()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(version),
                    )
                    .child(
                        div()
                            .rounded(t.radius)
                            .bg(t.field)
                            .px_2()
                            .py(px(2.))
                            .text_size(px(12.))
                            .child(channel),
                    )
                    .child(
                        div()
                            .w(px(51.))
                            .text_right()
                            .text_color(t.muted)
                            .child(date),
                    )
            }),
        )
    }
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = self.tokens;
        let width = window.viewport_size().width;
        let wide = width >= px(960.);
        let inset = px(if width < px(640.) { 14. } else { 24. });
        let main = div()
            .min_w_0()
            .flex()
            .flex_col()
            .when(wide, |v| v.flex_1())
            .child(self.sample.clone())
            .child(div().px(inset).child(self.history()));
        let side = div()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_4()
            .when(wide, |v| v.w(relative(0.31)).pt(inset).pr(inset))
            .when(!wide, |v| v.px(inset).pt_4())
            .child(self.checklist(cx))
            .child(self.notifications(cx));
        div()
            .id("create-gallery")
            .size_full()
            .overflow_y_scroll()
            .bg(t.canvas)
            .text_color(t.ink)
            .font_family(t.font)
            .text_size(px(14.))
            .child(
                div()
                    .mx_auto()
                    .w_full()
                    .max_w(px(1500.))
                    .flex()
                    .items_start()
                    .when(!wide, |v| v.flex_col().items_stretch())
                    .pb(inset)
                    .child(main)
                    .child(side),
            )
    }
}
