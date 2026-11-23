use super::palette::{Colors, DEPTH, RADIUS};
use gpui::{
    AnyView, App, AppContext as _, Context, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled as _, Window, div, px, rgb,
};
use gpui_deck::{Button, box_shadow};

pub(super) fn show(name: &'static str, light: Colors, dark: Colors, cx: &mut App) -> AnyView {
    cx.new(|_| Swatch {
        name,
        light,
        dark,
        night: false,
    })
    .into()
}

struct Swatch {
    name: &'static str,
    light: Colors,
    dark: Colors,
    night: bool,
}

impl Render for Swatch {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = if self.night { self.dark } else { self.light };
        let ink = rgb(colors.ink);
        let border = rgb(colors.border);
        let next = if self.night {
            "Use light colors"
        } else {
            "Use dark colors"
        };
        let switch = Button::new("switch-appearance")
            .accessibility_label(next)
            .px_3()
            .py_2()
            .rounded(px(RADIUS))
            .border_2()
            .border_color(border)
            .bg(rgb(colors.secondary))
            .text_color(rgb(colors.on_secondary))
            .child(next)
            .focus_visible(|s| s.underline())
            .on_click(cx.listener(|this, _, _, cx| {
                this.night = !this.night;
                cx.notify();
            }));
        let mut chart = div().flex().w_full().gap_1();
        for color in colors.chart {
            chart = chart.child(div().flex_1().h(px(26.)).rounded(px(2.)).bg(rgb(color)));
        }
        let card = div()
            .w_full()
            .max_w(px(390.))
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(colors.surface))
            .rounded(px(RADIUS))
            .border_2()
            .border_color(border)
            .shadow(vec![box_shadow(DEPTH, DEPTH, 0., 0., border.into())])
            .p_5()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(28.))
                    .child(self.name),
            )
            .child(
                div()
                    .rounded(px(RADIUS))
                    .px_3()
                    .py_3()
                    .bg(rgb(colors.main))
                    .text_color(rgb(colors.on_main))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Accent   #{:06X}", colors.main)),
            )
            .child(div().text_sm().child("Chart series"))
            .child(chart)
            .child(switch);
        div()
            .id("color-specimen")
            .size_full()
            .overflow_y_scroll()
            .bg(rgb(colors.paper))
            .text_color(ink)
            .child(
                div()
                    .w_full()
                    .min_h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p_4()
                    .child(card),
            )
    }
}
