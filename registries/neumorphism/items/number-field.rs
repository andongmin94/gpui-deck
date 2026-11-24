//! Editable numeric stepper with fractional steps or free decimal input.
mod button;
mod control_style;
mod input;
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::{Input, InputAccessibility, InputState, input::InputEvent};
use std::ops::RangeInclusive;

pub(crate) struct NumberChanged(pub f64);

/// A positive step anchors the grid at the minimum. None permits any typed
/// decimal and increments the current value by one.
struct NumberRange {
    min: f64,
    max: f64,
    step: Option<f64>,
    places: usize,
}

impl NumberRange {
    fn new(bounds: RangeInclusive<f64>, step: Option<f64>) -> Self {
        let (min, max) = bounds.into_inner();
        assert!(min.is_finite() && max.is_finite() && min <= max);
        assert!(step.is_none_or(|s| s.is_finite() && s > 0.));
        let places = Self::decimals(min).max(step.map_or(0, Self::decimals));
        Self {
            min,
            max,
            step,
            places,
        }
    }

    fn decimals(value: f64) -> usize {
        value
            .to_string()
            .split_once('.')
            .map_or(0, |(_, fraction)| fraction.len())
    }

    // Convert the grid result back to its decimal precision before exposing it
    // as a value or text (e.g. the third 0.1 step is 0.3, not 0.30000000000000004).
    fn rounded(&self, value: f64, places: usize) -> f64 {
        if value <= self.min {
            return self.min;
        }
        if value >= self.max {
            return self.max;
        }
        let result = format!("{value:.places$}")
            .parse::<f64>()
            .expect("formatted decimal");
        if result == 0. {
            0.
        } else {
            result.clamp(self.min, self.max)
        }
    }

    fn bounded(&self, value: f64) -> f64 {
        let value = value.clamp(self.min, self.max);
        if let Some(step) = self.step
            && value > self.min
            && value < self.max
        {
            let index = ((value - self.min) / step).round();
            if index.is_finite() {
                return self.rounded(index.mul_add(step, self.min), self.places);
            }
        }
        value
    }

    fn parse(&self, text: &str, previous: f64) -> f64 {
        text.trim()
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite())
            .map_or(previous, |v| self.bounded(v))
    }

    fn advance(&self, value: f64, increase: bool) -> f64 {
        let Some(step) = self.step else {
            return self.rounded(
                value + if increase { 1. } else { -1. },
                Self::decimals(value),
            );
        };
        let index = (value - self.min) / step;
        let closest = index.round();
        // Treat round-off at an exact grid point as exact; a nonaligned maximum
        // still steps back to the preceding allowed value.
        let index = if (index - closest).abs() <= f64::EPSILON * index.abs().max(1.) * 2. {
            closest
        } else {
            index
        };
        let next = if increase {
            index.floor() + 1.
        } else {
            index.ceil() - 1.
        };
        let next = next.mul_add(step, self.min);
        if next <= self.min {
            self.min
        } else if next >= self.max {
            self.max
        } else {
            self.rounded(next, self.places)
        }
    }
}

pub(crate) struct NumberField {
    editor: Entity<InputState>,
    value: f64,
    bounds: NumberRange,
    label: SharedString,
    palette: Palette,
    _editor_events: Subscription,
}
impl EventEmitter<NumberChanged> for NumberField {}
impl NumberField {
    pub(crate) fn new(
        label: impl Into<SharedString>,
        initial: f64,
        range: RangeInclusive<f64>,
        step: Option<f64>,
        palette: Palette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        assert!(initial.is_finite());
        let bounds = NumberRange::new(range, step);
        let value = bounds.bounded(initial);
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
            bounds,
            label,
            palette,
            _editor_events: sub,
        }
    }
    pub(crate) fn value(&self) -> f64 {
        self.value
    }
    fn draft(&self, cx: &App) -> f64 {
        self.bounds.parse(&self.editor.read(cx).value(), self.value)
    }
    fn commit(&mut self, value: f64, window: &mut Window, cx: &mut Context<Self>) {
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
        self.commit(self.bounds.advance(value, increment), window, cx);
    }
}
impl Render for NumberField {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let value = self.draft(cx);
        let dec = button::press_button_with_palette("decrease", "−", p)
            .accessibility_label("Decrease value")
            .disabled(value <= self.bounds.min)
            .w(px(40.))
            .px_0()
            .on_click(cx.listener(|s, _, w, cx| s.change(false, w, cx)));
        let inc = button::press_button_with_palette("increase", "+", p)
            .accessibility_label("Increase value")
            .disabled(value >= self.bounds.max)
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
    let field = cx.new(|cx| NumberField::new("Seats", 3., 1.0..=8.0, Some(1.), p, window, cx));
    control_style::sample(field, cx, |_, state, _, _| state.clone())
}

#[cfg(test)]
mod tests {
    use super::NumberRange;

    #[test]
    fn decimal_edits_and_steps_respect_the_grid_and_bounds() {
        let range = NumberRange::new(-0.2..=0.35, Some(0.1));
        let value = range.parse("0.19", 0.);
        assert_eq!(value, 0.2);
        assert_eq!(range.advance(value, true).to_string(), "0.3");
        assert_eq!(range.advance(0.3, true), 0.35);
        assert_eq!(range.advance(0.35, false), 0.3);
        assert_eq!(range.parse("-9.5", value), -0.2);
        assert_eq!(range.parse("NaN", value), value);
        let free = NumberRange::new(0.0..=5.0, None);
        assert_eq!(free.parse("1.375", 0.), 1.375);
        assert_eq!(free.advance(1.375, true), 2.375);
        assert_eq!(NumberRange::new(0.25..=0.75, None).advance(0.5, true), 0.75);
    }
}
