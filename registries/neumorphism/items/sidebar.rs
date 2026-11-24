mod palette;
mod style;

use super::Palette;
use super::recipes::Sidebar;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Entity, SharedString, div, px};
use gpui_deck::{CollapsibleState, TabItem, TabsState};
pub(crate) fn sidebar(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    sections: &Entity<TabsState>,
    rail: &Entity<CollapsibleState>,
    p: Palette,
) -> Sidebar {
    Sidebar::new(id, sections, label, p)
        .sheet_below(px(500.))
        .rail(rail)
        .border_1()
        .rounded(p.radius)
        .border_color(p.border)
        .bg(p.surface)
        .shadow(palette::raised(p))
        .p_2()
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Workspace {
        sections: Entity<TabsState>,
        rail: Entity<CollapsibleState>,
    }
    impl Render for Workspace {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let selected = self.sections.read(cx).selected_item().unwrap();
            style::stage(
                p,
                div()
                    .flex()
                    .gap_3()
                    .child(
                        sidebar(
                            "workspace",
                            "Workspace navigation",
                            &self.sections,
                            &self.rail,
                            p,
                        )
                        .group("Navigation", ["overview", "settings"]),
                    )
                    .child(style::surface(p).min_w_0().flex_1().child(
                        match selected.value().as_ref() {
                            "settings" => "Settings · Preferences for this workspace",

                            _ => "Review the most important information for your current project.",
                        },
                    )),
            )
        }
    }
    cx.new(|cx| {
        let sections = cx.new(|cx| {
            TabsState::new(
                [
                    TabItem::new("overview", "Overview"),
                    TabItem::new("settings", "Settings"),
                ],
                cx,
            )
        });
        let rail = cx.new(|cx| CollapsibleState::new(cx).default_open(true));
        cx.observe(&sections, |_, _, cx| cx.notify()).detach();
        cx.observe(&rail, |_, _, cx| cx.notify()).detach();
        Workspace { sections, rail }
    })
    .into()
}
