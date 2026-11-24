//! Neumorphic surfaces shared by the installable display items.
use super::{Palette, palette};
use gpui::{AnyElement, AnyView, App, Div, ElementId, SharedString, Window, div, prelude::*, px};
use gpui_deck::Button;

pub(super) fn stage(p: Palette, child: impl IntoElement) -> impl IntoElement {
    div()
        .id("neumorphism-stage")
        .size_full()
        .overflow_y_scroll()
        .bg(p.canvas)
        .text_color(p.foreground)
        .child(
            div()
                .min_h_full()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .p_5()
                .child(
                    div()
                        .min_w_0()
                        .w_full()
                        .max_w(px(680.))
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(child),
                ),
        )
}

#[allow(dead_code)] // Some items sit directly on the stage.
pub(super) fn surface(p: Palette) -> Div {
    div()
        .border_1()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .shadow(palette::raised(p))
        .p_5()
}

#[allow(dead_code)] // Only interactive consumers need an action.
pub(super) fn action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    p: Palette,
) -> Button {
    let label = label.into();
    Button::new(id)
        .accessibility_label(label.clone())
        .child(label)
        .h(px(36.))
        .px_3()
        .rounded(palette::control_radius(p))
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .cursor_pointer()
        .shadow(palette::raised(p))
        .focus_visible(|s| s.underline())
        .hover(move |s| s.bg(p.highlight))
        .active(move |s| s.shadow(palette::inset(p)))
        .styles(|s| s.disabled(|s| s.opacity(0.45)))
}

#[allow(dead_code)] // Stateful views own their own Render implementation.
pub(super) fn sample(cx: &mut App, draw: fn(Palette) -> AnyElement) -> AnyView {
    struct Sample(fn(Palette) -> AnyElement);
    impl Render for Sample {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            stage(p, (self.0)(p))
        }
    }
    cx.new(|_| Sample(draw)).into()
}
