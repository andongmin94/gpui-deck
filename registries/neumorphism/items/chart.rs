mod palette;
mod style;

use super::Palette;
use super::recipes::{ChartKind, ChartRecipe};
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, SharedString};
use gpui_deck::{ChartSeries, ChartState, Direction};

/// The same formatter labels the scale, keyboard inspection and exact-value table.
pub(crate) fn chart(
    id: impl Into<ElementId>,
    state: &Entity<ChartState>,
    kind: ChartKind,
    title: impl Into<SharedString>,
    category: impl Into<SharedString>,
    p: Palette,
    format: impl Fn(f64) -> String + 'static,
) -> ChartRecipe {
    let title = title.into();
    ChartRecipe::new(
        id,
        state,
        kind,
        Direction::Ltr,
        title.clone(),
        format!("{title} data"),
        category,
    )
    .series_colors(palette::chart_colors(p))
    .format_value(format)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Trends(Entity<ChartState>);
    impl Render for Trends {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            style::stage(
                p,
                style::surface(p).child(chart(
                    "orders",
                    &self.0,
                    ChartKind::Bar,
                    "Visits over time",
                    "Date",
                    p,
                    |n| format!("{n} visits"),
                )),
            )
        }
    }
    cx.new(|cx| {
        let s = cx.new(|cx| {
            ChartState::new(
                ["09-01", "09-02", "09-03"],
                [ChartSeries::new("Visits", [140., 220., 175.])],
                cx,
            )
        });
        cx.observe(&s, |_, _, cx| cx.notify()).detach();
        Trends(s)
    })
    .into()
}
