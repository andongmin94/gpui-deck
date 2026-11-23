//! Layout and surfaces for the Neobrutal collection's examples.
use super::{Palette, palette};
use gpui::{AnyElement, AnyView, App, Div, ElementId, SharedString, Window, div, prelude::*, px};
use gpui_deck::{Button, box_shadow};

pub(super) fn stage(p: Palette, child: impl IntoElement) -> impl IntoElement {
    div()
        .id("neo-stage")
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

pub(super) fn surface(p: Palette) -> Div {
    div()
        .border_2()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(p.surface)
        .text_color(p.foreground)
        .shadow(vec![box_shadow(4., 4., 0., 0., p.border)])
        .p_4()
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
        .rounded(p.radius)
        .border_2()
        .border_color(p.border)
        .bg(p.accent)
        .text_color(p.accent_foreground)
        .cursor_pointer()
        .shadow(vec![box_shadow(3., 3., 0., 0., p.border)])
        .focus_visible(|s| s.underline())
        .hover(|s| s.shadow(vec![box_shadow(1., 1., 0., 0., p.border)]))
        .active(|s| s.shadow_none())
        .styles(|s| s.disabled(|s| s.opacity(0.45)))
}

#[allow(dead_code)] // Stateful views own their own Render implementation.
pub(super) fn sample(cx: &mut App, draw: fn(Palette) -> AnyElement) -> AnyView {
    struct Sample(fn(Palette) -> AnyElement);
    impl Render for Sample {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::yellow();
            stage(p, (self.0)(p))
        }
    }
    cx.new(|_| Sample(draw)).into()
}
