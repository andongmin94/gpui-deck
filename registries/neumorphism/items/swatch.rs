use super::palette::{self, Preset};
use gpui::{prelude::*, *};
use gpui_deck::Button;
pub(super) fn show(preset: Preset, cx: &mut App) -> AnyView {
    cx.new(|_| Swatch {
        preset,
        dark: false,
    })
    .into()
}
struct Swatch {
    preset: Preset,
    dark: bool,
}
impl Render for Swatch {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.preset.palette(self.dark);
        let sample = div()
            .w_full()
            .max_w(px(380.))
            .rounded(p.radius)
            .bg(p.surface)
            .shadow(palette::raised(p))
            .p_6()
            .flex()
            .flex_col()
            .gap_4()
            .child(div().text_sm().text_color(p.muted).child("Neumorphism UI"))
            .child(
                div()
                    .text_size(px(32.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.preset.name),
            )
            .child(
                palette::well(p)
                    .p_3()
                    .text_color(p.muted)
                    .child("Inset surface"),
            )
            .child(
                div().flex().gap_2().children(
                    palette::chart_colors(p)
                        .map(|color| div().h(px(20.)).flex_1().rounded(px(6.)).bg(color)),
                ),
            )
            .child(
                Button::new("switch-palette")
                    .accessibility_label("Toggle dark appearance")
                    .aria_toggled(if self.dark {
                        Toggled::True
                    } else {
                        Toggled::False
                    })
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
                        cx.notify()
                    })),
            );
        div()
            .id("theme-sample")
            .size_full()
            .overflow_y_scroll()
            .bg(p.canvas)
            .text_color(p.foreground)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p_5()
                    .child(sample),
            )
    }
}
