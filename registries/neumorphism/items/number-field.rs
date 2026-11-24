//! Editable integer stepper. Arrow keys, typed values and buttons share the same bounds.
mod button;
mod control_style;
mod input;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Input, InputAccessibility, InputState, input::InputEvent};
use std::ops::RangeInclusive;

pub(crate) struct NumberChanged(pub i64);
pub(crate) struct NumberField {
    editor: Entity<InputState>,
    value: i64,
    min: i64,
    max: i64,
    step: i64,
    label: SharedString,
    palette: Palette,
    _editor_events: Subscription,
}
impl EventEmitter<NumberChanged> for NumberField {}
impl NumberField {
    pub(crate) fn new(
        label: impl Into<SharedString>,
        initial: i64,
        bounds: RangeInclusive<i64>,
        step: i64,
        palette: Palette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (min, max) = bounds.into_inner();
        assert!(
            min <= max && step > 0,
            "NumberField needs ordered bounds and a positive step"
        );
        let value = initial.clamp(min, max);
        let label = label.into();
        let editor = cx.new(|cx| {
            let mut edit = input::editor_with_palette(window, cx, "", Role::SpinButton, palette)
                .default_value(value.to_string());
            edit.set_accessibility(
                InputAccessibility::new(Some(Role::SpinButton)).with_label(Some(label.clone())),
            );
            edit
        });
        let sub =
            cx.subscribe_in(
                &editor,
                window,
                |this: &mut Self, _, event, window, cx| match event {
                    InputEvent::PressEnter { .. } | InputEvent::Blur => {
                        let value = this.draft(cx);
                        this.commit(value, window, cx);
                    }
                    InputEvent::Change => cx.notify(),
                    _ => {}
                },
            );
        Self {
            editor,
            value,
            min,
            max,
            step,
            label,
            palette,
            _editor_events: sub,
        }
    }
    pub(crate) fn value(&self) -> i64 {
        self.value
    }
    fn draft(&self, cx: &App) -> i64 {
        self.editor
            .read(cx)
            .value()
            .trim()
            .parse::<i64>()
            .unwrap_or(self.value)
            .clamp(self.min, self.max)
    }
    fn commit(&mut self, value: i64, window: &mut Window, cx: &mut Context<Self>) {
        let value = value.clamp(self.min, self.max);
        let draft = value.to_string();
        if self.editor.read(cx).value().as_ref() != draft {
            self.editor
                .update(cx, |s, cx| s.set_value(draft, window, cx));
        }
        if value != self.value {
            self.value = value;
            cx.emit(NumberChanged(value));
        }
        cx.notify();
    }
    fn change(&mut self, increment: bool, window: &mut Window, cx: &mut Context<Self>) {
        let value = self.draft(cx);
        self.commit(
            if increment {
                value.saturating_add(self.step)
            } else {
                value.saturating_sub(self.step)
            },
            window,
            cx,
        );
    }
}
impl Render for NumberField {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let value = self.draft(cx);
        let dec = button::press_button_with_palette("decrease", "−", p)
            .accessibility_label("Decrease value")
            .disabled(value <= self.min)
            .w(px(40.))
            .px_0()
            .on_click(cx.listener(|s, _, w, cx| s.change(false, w, cx)));
        let inc = button::press_button_with_palette("increase", "+", p)
            .accessibility_label("Increase value")
            .disabled(value >= self.max)
            .w(px(40.))
            .px_0()
            .on_click(cx.listener(|s, _, w, cx| s.change(true, w, cx)));
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(self.label.clone())
            .child(
                div()
                    .id("stepper")
                    .flex()
                    .items_center()
                    .gap_2()
                    .on_key_down(cx.listener(|s, e: &KeyDownEvent, w, cx| {
                        match e.keystroke.key.as_str() {
                            "up" => s.change(true, w, cx),
                            "down" => s.change(false, w, cx),
                            _ => return,
                        }
                        cx.stop_propagation();
                    }))
                    .child(dec)
                    .child(
                        palette::well(p)
                            .rounded(palette::control_radius(p))
                            .w(px(76.))
                            .px_3()
                            .py_2()
                            .child(Input::new(&self.editor)),
                    )
                    .child(inc),
            )
    }
}
pub(crate) fn preview(window: &mut Window, cx: &mut App) -> AnyView {
    let p = palette::air();
    let field = cx.new(|cx| NumberField::new("Seats", 3, 1..=100, 1, p, window, cx));
    control_style::sample(field, cx, |_, state, _, _| state.clone())
}
