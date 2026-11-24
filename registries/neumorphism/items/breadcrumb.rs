mod palette;
mod style;

use super::Palette;
use gpui::{AnyView, App, Window, prelude::*};
use gpui::{Div, ElementId, SharedString, div};
use gpui_deck::{Breadcrumb, BreadcrumbItem, BreadcrumbList, Button};

/// The last segment is current. Activate an ancestor to navigate to its index.
pub(crate) fn breadcrumb(
    id: impl Into<ElementId>,
    path: &[SharedString],
    p: Palette,
    on_navigate: impl Fn(usize, &mut Window, &mut App) + 'static,
) -> Breadcrumb {
    let id = id.into();
    let navigate = std::rc::Rc::new(on_navigate);
    let mut list = BreadcrumbList::new((id.clone(), "list"))
        .flex()
        .flex_wrap()
        .items_center()
        .gap_2();
    for (index, label) in path.iter().enumerate() {
        let current = index + 1 == path.len();
        let content: Div = if current {
            div()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child(label.clone())
        } else {
            let navigate = navigate.clone();
            div().child(
                Button::new((id.clone(), format!("link-{index}")))
                    .accessibility_label(label.clone())
                    .child(label.clone())
                    .underline()
                    .focus_visible(|s| s.bg(p.highlight))
                    .on_click(move |_, window, cx| navigate(index, window, cx)),
            )
        };
        list = list.child(
            BreadcrumbItem::new((id.clone(), format!("item-{index}")))
                .current(current)
                .flex()
                .items_center()
                .gap_2()
                .when(index != 0, |s| s.child("›"))
                .child(content),
        );
    }
    Breadcrumb::new(id).text_color(p.foreground).child(list)
}
pub(crate) fn preview(_: &mut Window, cx: &mut App) -> AnyView {
    struct Trail(usize);
    impl Render for Trail {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let p = palette::air();
            let path = ["Library".into(), "Components".into(), "Breadcrumb".into()];
            let owner = cx.entity();
            style::stage(
                p,
                style::surface(p)
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap_3()
                    .child(breadcrumb("path", &path[..self.0], p, move |idx, _, cx| {
                        owner.update(cx, |this, cx| {
                            this.0 = idx + 1;
                            cx.notify();
                        });
                    }))
                    .when(self.0 < path.len(), |s| {
                        s.child(
                            style::action("open", "Open breadcrumb example", p).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.0 = 3;
                                    cx.notify();
                                }),
                            ),
                        )
                    }),
            )
        }
    }
    cx.new(|_| Trail(3)).into()
}
