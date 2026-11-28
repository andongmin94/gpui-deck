use gpui::{Context, Entity, Subscription, Window, div, prelude::*, px, rgb};
use gpui_deck::{Root, Select, SelectItem, SelectOption, SelectState};

struct TimeZonePicker {
    zones: Entity<SelectState>,
    _observation: Subscription,
}

impl TimeZonePicker {
    fn new(cx: &mut Context<Self>) -> Self {
        let zones = cx.new(|cx| {
            SelectState::new(
                [
                    SelectItem::new("America/Chicago", "Chicago"),
                    SelectItem::new("Europe/London", "London"),
                    SelectItem::new("Europe/Paris", "Paris").disabled(true),
                    SelectItem::new("Asia/Seoul", "Seoul"),
                    SelectItem::new("Asia/Tokyo", "Tokyo"),
                    SelectItem::new("Australia/Sydney", "Sydney"),
                ],
                cx,
            )
            .default_value("Asia/Seoul")
        });
        let observation = cx.observe(&zones, |_, _, cx| cx.notify());
        Self {
            zones,
            _observation: observation,
        }
    }
}

impl Render for TimeZonePicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.zones.read(cx);
        let label = state
            .selected_item()
            .map(|item| item.label().clone())
            .unwrap_or_else(|| "Choose a city".into());
        let options = state.items().iter().enumerate().map(|(index, city)| {
            SelectOption::new(&self.zones, index)
                .h(px(32.))
                .px_3()
                .when(state.highlighted_index() == Some(index), |row| {
                    row.bg(rgb(0xe4e4e7))
                })
                .when(city.is_disabled(), |row| row.opacity(0.45))
                .child(city.label().clone())
        });

        div()
            .size_full()
            .p_6()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x18181b))
            .child(
                Select::new("zone", &self.zones)
                    .accessibility_label("Time zone")
                    .w(px(220.))
                    .list_max_height(px(96.))
                    .border_1()
                    .rounded_md()
                    .focus_visible(|style| style.border_color(rgb(0x2563eb)))
                    .trigger(div().px_3().py_2().child(label))
                    .content(
                        div()
                            .w(px(220.))
                            .bg(rgb(0xffffff))
                            .border_1()
                            .children(options),
                    ),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        gpui_deck::init(cx);
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.open_window(Default::default(), |window, cx| {
            let screen = cx.new(TimeZonePicker::new);
            cx.new(|cx| Root::new(screen, window, cx))
        })
        .expect("open window");
        cx.activate(true);
    });
}
