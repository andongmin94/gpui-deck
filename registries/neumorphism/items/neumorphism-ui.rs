//! Foundation styles and default Air palette for a source-owned Neumorphism UI app.
mod palette;
use super::Palette;
use gpui::{prelude::*, *};
use gpui_deck::Button;

pub(crate) fn palette(dark: bool) -> Palette {
    palette::AIR.palette(dark)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| Foundation {
        dark: false,
        pressed: false,
    })
    .into()
}
struct Foundation {
    dark: bool,
    pressed: bool,
}
impl Render for Foundation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = palette(self.dark);
        let mut sample = Button::new("depth")
            .accessibility_label("Press the sample surface")
            .aria_toggled(if self.pressed {
                Toggled::True
            } else {
                Toggled::False
            })
            .w_full()
            .h(px(72.))
            .rounded(p.radius)
            .bg(p.surface)
            .text_color(p.foreground)
            .focus_visible(|s| s.underline())
            .child(if self.pressed {
                "Inset surface"
            } else {
                "Raised surface"
            })
            .on_click(cx.listener(|this, _, _, cx| {
                this.pressed = !this.pressed;
                cx.notify();
            }));
        sample = sample.shadow(if self.pressed {
            palette::inset(p)
        } else {
            palette::raised(p)
        });
        let card = div()
            .w_full()
            .max_w(px(430.))
            .p_6()
            .bg(p.surface)
            .rounded(p.radius)
            .shadow(palette::raised(p))
            .flex()
            .flex_col()
            .gap_5()
            .child(div().text_sm().text_color(p.muted).child("Neumorphism UI"))
            .child(
                div()
                    .text_size(px(30.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Air foundation"),
            )
            .child(sample)
            .child(palette::well(p).p_3().child("Recessed field"))
            .child(
                Button::new("switch-palette")
                    .accessibility_label("Toggle dark appearance")
                    .bg(p.accent)
                    .text_color(p.accent_foreground)
                    .rounded(palette::control_radius(p))
                    .px_3()
                    .py_2()
                    .focus_visible(|s| s.underline())
                    .child(if self.dark {
                        "Use light colors"
                    } else {
                        "Use dark colors"
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.dark = !this.dark;
                        cx.notify();
                    })),
            );
        div()
            .id("foundation")
            .size_full()
            .overflow_y_scroll()
            .bg(p.canvas)
            .text_color(p.foreground)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .p_6()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(card),
            )
    }
}
