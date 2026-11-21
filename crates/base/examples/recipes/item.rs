//! Application-owned content row with optional media, header, footer and
//! consumer-supplied trailing action.

use gpui::{
    AnyElement, App, Div, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Role, SharedString, Stateful, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
};

use gpui_deck::{ObservedElement, TestSupportExt as _};

use super::super::Palette;

/// A named row with static text and optional media, header, footer, and an
/// action or control.
///
/// The row adds no focus stop, click handler, or selection state. Supplied
/// controls keep their own accessible names and keyboard/pointer behavior.
/// Media sits before the text and keeps its width; it carries only the
/// semantics of what you supply, so give an informative image its own name and
/// leave a decorative shape unnamed. The header and footer span the full
/// width above and below the row. Use a container when a slot needs several
/// elements.
#[derive(IntoElement)]
pub(crate) struct Item {
    base: ObservedElement<Stateful<Div>>,
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    media: Option<AnyElement>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    action: Option<AnyElement>,
    palette: Palette,
}

impl Item {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        palette: Palette,
    ) -> Self {
        let id = id.into();
        Self {
            base: div()
                .id(id.clone())
                .test_support()
                .flex()
                .flex_col()
                .gap(palette.gap)
                .px_3()
                .py_2()
                .border_1()
                .border_color(palette.border)
                .rounded(palette.radius)
                .text_size(palette.font_size)
                .text_color(palette.foreground),
            id,
            title: title.into(),
            description: None,
            media: None,
            header: None,
            footer: None,
            action: None,
            palette,
        }
    }

    pub(crate) fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub(crate) fn media(mut self, media: impl IntoElement) -> Self {
        self.media = Some(media.into_any_element());
        self
    }

    pub(crate) fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub(crate) fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub(crate) fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl Styled for Item {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Item {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let id = self.id;
        let text = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .id((id.clone(), "title"))
                    .test_support()
                    .role(Role::Label)
                    .aria_label(self.title.clone())
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.title.clone()),
            )
            .when_some(self.description.clone(), |text, description| {
                text.child(
                    div()
                        .id((id.clone(), "description"))
                        .role(Role::Label)
                        .aria_label(description.clone())
                        .text_sm()
                        .text_color(self.palette.muted)
                        .child(description),
                )
            });
        let slot = |name: &'static str, content: AnyElement| {
            div()
                .id((id.clone(), name))
                .test_support()
                .flex()
                .items_center()
                .flex_shrink_0()
                .child(content)
        };
        let row = div()
            .flex()
            .items_center()
            .gap(self.palette.gap)
            .when_some(self.media, |row, media| row.child(slot("media", media)))
            .child(text)
            .when_some(self.action, |row, action| row.child(slot("action", action)));

        self.base
            .role(Role::Group)
            .aria_label(self.title)
            .when_some(self.description, |item, description| {
                item.aria_description(description)
            })
            .when_some(self.header, |item, header| {
                item.child(
                    div()
                        .id((id.clone(), "header"))
                        .test_support()
                        .min_w_0()
                        .child(header),
                )
            })
            .child(row)
            .when_some(self.footer, |item, footer| {
                item.child(
                    div()
                        .id((id.clone(), "footer"))
                        .test_support()
                        .min_w_0()
                        .child(footer),
                )
            })
    }
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use gpui::{AppContext as _, Context, Render, TestAppContext, px, size};
    use gpui_deck::{Button, Root, test_support};

    use super::super::super::Appearance;
    use super::*;

    struct Sample;

    impl Render for Sample {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(320.)).child(
                Item::new(
                    "sample-item",
                    "Recording preset",
                    Appearance::Compact.palette(),
                )
                .description("Balanced quality")
                .media(div().size(px(32.)))
                .header("Applies to new recordings")
                .footer("Recording date: Not scheduled")
                .action(Button::new("sample-edit").child("Edit")),
            )
        }
    }

    #[gpui::test]
    fn slots_surround_the_named_row_without_semantics_or_focus(cx: &mut TestAppContext) {
        cx.update(gpui_deck::init);
        let (_, cx) = cx.add_window_view(|window, cx| {
            let sample = cx.new(|_| Sample);
            Root::new(sample, window, cx)
        });
        cx.simulate_resize(size(px(400.), px(400.)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, _| {
            let item = ElementId::from("sample-item");
            let part = |name: &str| {
                test_support::find(window, &[], &(item.clone(), name.to_owned()).into()).unwrap()
            };
            let frame = test_support::find(window, &[], &item).unwrap();
            assert_eq!(frame.role(), Some(Role::Group));
            assert_eq!(frame.label(), Some("Recording preset"));
            assert_eq!(frame.description(), Some("Balanced quality"));
            let (header, media, title, action, footer) = (
                part("header").bounds(),
                part("media").bounds(),
                part("title").bounds(),
                part("action").bounds(),
                part("footer").bounds(),
            );
            assert!(header.bottom() <= media.top() && header.bottom() <= title.top());
            assert!(footer.top() >= media.bottom() && footer.top() >= title.bottom());
            assert!(media.right() <= title.left(), "media leads the text");
            assert!(action.left() >= title.right(), "the action trails the text");
            assert_eq!(media.size.width, px(32.), "media keeps its width");
            for bounds in [header, media, action, footer] {
                assert!(bounds.left() >= frame.bounds().left());
                assert!(bounds.right() <= frame.bounds().right() + px(1.));
            }
            for name in ["header", "media", "action", "footer"] {
                let slot = part(name);
                assert_eq!(slot.role(), None);
                assert_eq!(slot.focused(), None);
            }
            let focusable: Vec<_> = test_support::snapshots(window)
                .into_iter()
                .filter(|node| node.path().contains(&item) && node.focused().is_some())
                .collect();
            assert_eq!(focusable.len(), 1, "only the supplied action is focusable");
        });
    }
}
