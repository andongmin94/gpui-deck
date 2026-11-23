//! Interactive report with the same chart and exact-value table over one selected data set.
use super::super::recipes::{ChartKind, ChartRecipe};
use super::{Palette, colors::series_colors, style};
use gpui::{AnyView, App, Entity, Subscription, Window, div, prelude::*, px};
use gpui_deck::{ChartSeries, ChartState, Direction};

pub(crate) struct Filter {
    pub(crate) label: &'static str,
    pub(crate) options: &'static [&'static str],
}

pub(crate) struct SeriesData {
    pub(crate) categories: Vec<&'static str>,
    pub(crate) series: Vec<ChartSeries>,
    pub(crate) kind: ChartKind,
    pub(crate) format: fn(f64) -> String,
    pub(crate) summary: String,
}

pub(crate) struct Report {
    title: &'static str,
    category: &'static str,
    filters: &'static [Filter],
    selected: Vec<usize>,
    dataset: fn(&[usize]) -> SeriesData,
    palette: Palette,
    state: Entity<ChartState>,
    data: SeriesData,
    observer: Subscription,
}

impl SeriesData {
    fn mount(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Report>,
    ) -> (Entity<ChartState>, Subscription) {
        let state = cx.new(|cx| {
            ChartState::new(
                self.categories.iter().copied(),
                std::mem::take(&mut self.series),
                cx,
            )
        });
        state.update(cx, |s, cx| s.set_layout(self.kind.layout(), window, cx));
        let observer = cx.observe(&state, |_, _, cx| cx.notify());
        (state, observer)
    }
}

impl Report {
    /// Filters index their corresponding entries in `dataset` in display order.
    pub(crate) fn open(
        title: &'static str,
        category: &'static str,
        filters: &'static [Filter],
        dataset: fn(&[usize]) -> SeriesData,
        palette: Palette,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| {
            let selected = vec![0; filters.len()];
            let mut data = dataset(&selected);
            let (state, observer) = data.mount(window, cx);
            Self {
                title,
                category,
                filters,
                selected,
                dataset,
                palette,
                state,
                data,
                observer,
            }
        })
        .into()
    }

    fn select(&mut self, group: usize, value: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected[group] == value {
            return;
        }
        self.selected[group] = value;
        self.data = (self.dataset)(&self.selected);
        (self.state, self.observer) = self.data.mount(window, cx);
        cx.notify();
    }
}

impl Render for Report {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette;
        let mut filters = div().flex().flex_wrap().gap_3();
        for (group, filter) in self.filters.iter().enumerate() {
            let mut options = div().flex().flex_wrap().gap_1();
            for (index, &label) in filter.options.iter().enumerate() {
                let selected = self.selected[group] == index;
                options = options.child(
                    style::action((filter.label, index), label, p)
                        .selected(selected)
                        .bg(if selected { p.accent } else { p.surface })
                        .text_color(if selected {
                            p.accent_foreground
                        } else {
                            p.foreground
                        })
                        .accessibility_label(format!(
                            "{}: {label}{}",
                            filter.label,
                            if selected { " (selected)" } else { "" }
                        ))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.select(group, index, window, cx)
                        })),
                );
            }
            filters = filters.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().text_sm().child(filter.label))
                    .child(options),
            );
        }
        let chart = ChartRecipe::new(
            "report-plot",
            &self.state,
            self.data.kind,
            Direction::Ltr,
            self.title,
            format!("{} data", self.title),
            self.category,
        )
        .series_colors(series_colors(p))
        .format_value(self.data.format);
        style::stage(
            p,
            style::surface(p)
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_size(px(24.))
                        .child(self.title),
                )
                .child(filters)
                .child(
                    div()
                        .id("summary")
                        .role(gpui::Role::Status)
                        .aria_label(self.data.summary.clone())
                        .child(self.data.summary.clone()),
                )
                .child(chart),
        )
    }
}
