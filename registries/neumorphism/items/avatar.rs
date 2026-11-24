mod palette;
mod style;

use super::{
    Palette,
    recipes::{Avatar, AvatarGroup},
};
use gpui::{prelude::*, *};

pub(crate) fn avatar(
    id: impl Into<ElementId>,
    name: impl Into<SharedString>,
    initials: impl Into<SharedString>,
    p: Palette,
) -> Avatar {
    Avatar::new(id, name, initials, p)
        .border_1()
        .border_color(p.border)
        .bg(p.surface)
        .shadow(palette::raised(p))
        .text_color(p.foreground)
}

pub(crate) fn with_status(
    id: impl Into<ElementId>,
    avatar: Avatar,
    status: impl Into<SharedString>,
    p: Palette,
) -> Div {
    div().relative().child(avatar).child(
        div()
            .id(id)
            .role(Role::Label)
            .aria_label(status)
            .absolute()
            .bottom_0()
            .right_0()
            .size(px(12.))
            .rounded_full()
            .border_2()
            .border_color(p.surface)
            .bg(palette::chart_colors(p)[2]),
    )
}

pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    style::sample(cx, |p| {
        let team = AvatarGroup::new("team", "Team", p)
            .avatar(avatar("an", "AN", "AN", p))
            .avatar(avatar("de", "DE", "DE", p))
            .avatar(avatar("so", "SO", "SO", p));
        let members = div().flex().items_center().child(team).child(
            palette::well(p)
                .id("remaining")
                .role(Role::Label)
                .aria_label("9 more team members")
                .size(px(40.))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .child("+9"),
        );
        style::surface(p)
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(avatar("sm", "SM", "SM", p).size(px(32.)))
                    .child(with_status(
                        "am-status",
                        avatar("am", "AM", "AM", p).size(px(40.)),
                        "Online",
                        p,
                    ))
                    .child(with_status(
                        "ui-status",
                        avatar("ui", "UI", "UI", p).size(px(52.)),
                        "Online",
                        p,
                    )),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child("Team")
                    .child(members),
            )
            .into_any_element()
    })
}
