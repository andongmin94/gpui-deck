mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{ElementId, Image, ImageFormat, ImageSource, ObjectFit, SharedString, div, img, px};
use std::sync::{Arc, LazyLock};

pub(crate) fn image_card(
    id: impl Into<ElementId>,
    source: impl Into<ImageSource>,
    alt: impl Into<SharedString>,
    title: impl Into<SharedString>,
    caption: impl Into<SharedString>,
    p: Palette,
) -> impl IntoElement {
    let id = id.into();
    style::surface(p)
        .w_full()
        .max_w(px(340.))
        .mx_auto()
        .p_0()
        .flex()
        .flex_col()
        .overflow_hidden()
        .child(
            div()
                .relative()
                .flex_none()
                .w_full()
                .h(px(150.))
                .overflow_hidden()
                .child(
                    img(source)
                        .id((id.clone(), "image"))
                        .role(gpui::Role::Image)
                        .aria_label(alt)
                        .absolute()
                        .inset_0()
                        .size_full()
                        .object_fit(ObjectFit::Cover),
                ),
        )
        .child(
            div()
                .flex_none()
                .p_4()
                .border_t_1()
                .border_color(p.border)
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .id((id, "title"))
                        .role(gpui::Role::Heading)
                        .aria_level(2)
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(title.into()),
                )
                .child(caption.into()),
        )
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    static ART: LazyLock<Arc<Image>> = LazyLock::new(|| {
        Arc::new(Image::from_bytes(
            ImageFormat::Svg,
            include_bytes!("workspace.svg").to_vec(),
        ))
    });
    style::sample(cx, |p| {
        image_card(
            "workspace",
            ART.clone(),
            "Workspace cover with a soft blue gradient",
            "Design system workspace",
            "12 components",
            p,
        )
        .into_any_element()
    })
}
