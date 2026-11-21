//! Buttons, toggles and other single-value controls.

use gpui::{
    AnyView, App, AppContext as _, Context, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, Styled as _, Subscription, Window, div,
    prelude::FluentBuilder as _, px, relative,
};
use gpui_deck::{
    Button, Checkbox, CheckboxState, Progress, RadioGroup, RadioGroupState, RadioItem, RadioOption,
    Separator, Slider, SliderState, SliderThumb, Switch, Toggle, ToggleGroup, ToggleGroupState,
    ToggleItem, ToggleOption,
};

use super::{frame, outline};
use crate::recipes::ButtonVariant;
use crate::theme::Theme;

// Button

struct ButtonDemo {
    theme: Theme,
    last_pressed: Option<&'static str>,
}

impl Render for ButtonDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let palette = theme.recipe_palette();
        let variants = [
            ("Primary", ButtonVariant::Primary),
            ("Secondary", ButtonVariant::Secondary),
            ("Outline", ButtonVariant::Outline),
            ("Ghost", ButtonVariant::Ghost),
            ("Destructive", ButtonVariant::Destructive),
            ("Link", ButtonVariant::Link),
        ];
        let buttons = div()
            .flex()
            .flex_wrap()
            .gap_2()
            .justify_center()
            .children(variants.into_iter().map(|(label, variant)| {
                variant
                    .apply(Button::new(label).accessibility_label(label), palette)
                    .child(label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.last_pressed = Some(label);
                        cx.notify();
                    }))
            }))
            .child(
                ButtonVariant::Primary
                    .apply(
                        Button::new("disabled")
                            .accessibility_label("Disabled")
                            .disabled(true),
                        palette,
                    )
                    .child("Disabled"),
            );
        frame(theme)
            .bg(palette.canvas)
            .text_color(palette.foreground)
            .flex_col()
            .child(buttons)
            .when_some(self.last_pressed, |view, label| {
                view.child(format!("{label} pressed"))
            })
    }
}

pub fn button(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| ButtonDemo {
        theme,
        last_pressed: None,
    })
    .into()
}

// Toggle and ToggleGroup

struct ToggleDemo {
    theme: Theme,
    bold: bool,
    group: Entity<ToggleGroupState>,
    _observe: Subscription,
}

impl Render for ToggleDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let group = self.group.read(cx);
        let options = (0..group.items().len()).map(|index| {
            let pressed = group.is_pressed(index);
            ToggleOption::new(&self.group, index)
                .h(px(36.))
                .px(px(12.))
                .flex()
                .items_center()
                .rounded(px(6.))
                .when(pressed, |option| option.bg(theme.accent))
                .hover(|style| style.bg(theme.accent))
                .focus_visible(|style| style.border_1().border_color(theme.ring))
                .child(group.items()[index].label().clone())
        });
        let entity = cx.entity();
        frame(theme)
            .flex_col()
            .child(
                Toggle::new("bold")
                    .accessibility_label("Bold")
                    .pressed(self.bold)
                    .h(px(36.))
                    .px(px(12.))
                    .flex()
                    .items_center()
                    .rounded(px(6.))
                    .font_weight(gpui::FontWeight::BOLD)
                    .when(self.bold, |toggle| toggle.bg(theme.accent))
                    .hover(|style| style.bg(theme.accent))
                    .focus_visible(|style| style.text_color(theme.primary).underline())
                    .child("B")
                    .on_change(move |pressed, _, _, cx| {
                        entity.update(cx, |this, cx| {
                            this.bold = pressed;
                            cx.notify();
                        })
                    }),
            )
            .child(
                ToggleGroup::new("alignment", &self.group)
                    .accessibility_label("Text alignment")
                    .flex()
                    .gap(px(4.))
                    .p(px(4.))
                    .rounded(px(8.))
                    .border_1()
                    .border_color(theme.border)
                    .children(options),
            )
    }
}

pub fn toggle(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let group = cx.new(|cx| {
            ToggleGroupState::new(
                [
                    ToggleItem::new("left", "Left"),
                    ToggleItem::new("center", "Center"),
                    ToggleItem::new("right", "Right"),
                ],
                cx,
            )
            .single_selection()
        });
        let observe = cx.observe(&group, |_, _, cx| cx.notify());
        ToggleDemo {
            theme,
            bold: true,
            group,
            _observe: observe,
        }
    })
    .into()
}

// Checkbox and Switch

struct CheckboxDemo {
    theme: Theme,
    terms: CheckboxState,
    airplane: bool,
}

impl Render for CheckboxDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let checked = self.terms == CheckboxState::Checked;
        let terms = cx.entity();
        let airplane = cx.entity();
        frame(theme)
            .flex_col()
            .items_start()
            .child(
                Checkbox::new("terms")
                    .state(self.terms)
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .focus_visible(|style| style.text_color(theme.primary).underline())
                    .child(
                        div()
                            .size(px(16.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.))
                            .border_1()
                            .border_color(if checked {
                                theme.primary
                            } else {
                                theme.control_border
                            })
                            .when(checked, |indicator| {
                                indicator
                                    .bg(theme.primary)
                                    .text_color(theme.primary_foreground)
                                    .text_size(px(11.))
                                    .child("✓")
                            }),
                    )
                    .child("Accept terms and conditions")
                    .on_change(move |state, _, _, cx| {
                        terms.update(cx, |this, cx| {
                            this.terms = state;
                            cx.notify();
                        })
                    }),
            )
            .child(
                Switch::new("airplane")
                    .accessibility_label("Airplane mode")
                    .checked(self.airplane)
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .focus_visible(|style| style.text_color(theme.primary).underline())
                    .child(
                        div()
                            .w(px(36.))
                            .h(px(20.))
                            .p(px(2.))
                            .flex()
                            .rounded_full()
                            .bg(if self.airplane {
                                theme.primary
                            } else {
                                theme.border
                            })
                            .when(self.airplane, |track| track.justify_end())
                            .child(div().size(px(16.)).rounded_full().bg(theme.background)),
                    )
                    .child("Airplane mode")
                    .on_change(move |checked, _, _, cx| {
                        airplane.update(cx, |this, cx| {
                            this.airplane = checked;
                            cx.notify();
                        })
                    }),
            )
    }
}

pub fn checkbox(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| CheckboxDemo {
        theme,
        terms: CheckboxState::Unchecked,
        airplane: false,
    })
    .into()
}

// RadioGroup

struct RadioDemo {
    theme: Theme,
    state: Entity<RadioGroupState>,
    _observe: Subscription,
}

impl Render for RadioDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let state = self.state.read(cx);
        let selected = state.selected_index();
        let options = state.items().iter().enumerate().map(|(index, item)| {
            let checked = selected == Some(index);
            RadioOption::new(&self.state, index)
                .flex()
                .items_center()
                .gap(px(8.))
                .focus_visible(|style| style.text_color(theme.primary).underline())
                .child(
                    div()
                        .size(px(16.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .border_1()
                        .border_color(if checked {
                            theme.primary
                        } else {
                            theme.control_border
                        })
                        .when(checked, |ring| {
                            ring.child(div().size(px(8.)).rounded_full().bg(theme.primary))
                        }),
                )
                .child(item.label().clone())
        });
        frame(theme).child(
            RadioGroup::new("density", &self.state)
                .accessibility_label("Density")
                .flex()
                .flex_col()
                .gap(px(12.))
                .children(options),
        )
    }
}

pub fn radio_group(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let state = cx.new(|cx| {
            RadioGroupState::new(
                [
                    RadioItem::new("default", "Default"),
                    RadioItem::new("comfortable", "Comfortable"),
                    RadioItem::new("compact", "Compact"),
                ],
                cx,
            )
            .default_value("comfortable")
        });
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        RadioDemo {
            theme,
            state,
            _observe: observe,
        }
    })
    .into()
}

// Slider

struct SliderDemo {
    theme: Theme,
    single: Entity<SliderState>,
    range: Entity<SliderState>,
    _observe: [Subscription; 2],
}

fn slider_rail(
    theme: Theme,
    id: &'static str,
    label: &'static str,
    state: &Entity<SliderState>,
    cx: &App,
) -> Slider {
    let model = state.read(cx);
    let count = model.values().len();
    let (start, end) = if count == 1 {
        (0., model.percentage(0))
    } else {
        (model.percentage(0), model.percentage(count - 1))
    };
    let mut rail = Slider::new(id, state)
        .accessibility_label(label)
        .relative()
        .w(px(240.))
        .h(px(20.))
        .child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .top(px(8.))
                .h(px(4.))
                .rounded_full()
                .bg(theme.muted),
        )
        .child(
            div()
                .absolute()
                .left(relative(start))
                .w(relative(end - start))
                .top(px(8.))
                .h(px(4.))
                .rounded_full()
                .bg(theme.primary),
        );
    for index in 0..count {
        rail = rail.child(
            SliderThumb::new(state, index)
                .accessibility_label(label)
                .absolute()
                .left(relative(model.percentage(index)))
                .ml(px(-8.))
                .top(px(2.))
                .size(px(16.))
                .rounded_full()
                .border_1()
                .border_color(theme.primary)
                .bg(theme.background)
                .focus_visible(|style| style.border_2()),
        );
    }
    rail
}

impl Render for SliderDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        let single = self.single.read(cx).values()[0];
        let range = self.range.read(cx).values().to_vec();
        frame(theme)
            .flex_col()
            .gap(px(20.))
            .child(slider_rail(theme, "volume", "Volume", &self.single, cx))
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child(format!("Volume {single:.0}")),
            )
            .child(slider_rail(theme, "price", "Price range", &self.range, cx))
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child(format!("Range {:.0} – {:.0}", range[0], range[1])),
            )
    }
}

pub fn slider(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let single = cx.new(|cx| SliderState::new(0.0..=100.0, 1., [50.], cx));
        let range = cx.new(|cx| SliderState::new(0.0..=100.0, 5., [25., 75.], cx));
        let observe = [
            cx.observe(&single, |_, _, cx| cx.notify()),
            cx.observe(&range, |_, _, cx| cx.notify()),
        ];
        SliderDemo {
            theme,
            single,
            range,
            _observe: observe,
        }
    })
    .into()
}

// Progress

struct ProgressDemo {
    theme: Theme,
    value: f32,
}

impl Render for ProgressDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme)
            .flex_col()
            .child(
                Progress::new("upload")
                    .accessibility_label("Upload progress")
                    .value(self.value)
                    .w(px(280.))
                    .h(px(8.))
                    .rounded_full()
                    .bg(theme.muted)
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(relative(self.value / 100.))
                            .bg(theme.primary),
                    ),
            )
            .child(
                outline(theme, Button::new("advance"))
                    .child(format!("{:.0}% · Advance", self.value))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.value = if this.value >= 100. {
                            0.
                        } else {
                            this.value + 20.
                        };
                        cx.notify();
                    })),
            )
    }
}

pub fn progress(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| ProgressDemo { theme, value: 40. }).into()
}

// Separator

struct SeparatorDemo {
    theme: Theme,
}

impl Render for SeparatorDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.resolve(cx);
        frame(theme).child(
            div()
                .w(px(280.))
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    div()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child("GPUI Deck"),
                )
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("Headless components for GPUI."),
                )
                .child(Separator::new("rule").h(px(1.)).w_full().bg(theme.border))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.))
                        .h(px(20.))
                        .child("Docs")
                        .child(
                            Separator::new("a")
                                .decorative(true)
                                .w(px(1.))
                                .h_full()
                                .bg(theme.border),
                        )
                        .child("Components")
                        .child(
                            Separator::new("b")
                                .decorative(true)
                                .w(px(1.))
                                .h_full()
                                .bg(theme.border),
                        )
                        .child("Recipes"),
                ),
        )
    }
}

pub fn separator(theme: Theme, cx: &mut App) -> AnyView {
    cx.new(|_| SeparatorDemo { theme }).into()
}
