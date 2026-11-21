//! Application-controlled numbered navigation over a known page count.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Role, SharedString, Stateful, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _,
};
use gpui_deck::{Button, ObservedElement, TestSupportExt as _};

use super::super::Palette;

type PageChange = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// The most page entries shown at once, including ellipses.
const VISIBLE: usize = 7;

/// Zero-based pages to show, with `None` for an elided run. The first and last
/// pages and the current page's neighbors stay visible, and the entry count is
/// constant once the collection exceeds [`VISIBLE`] pages, so controls do not
/// shift while paging.
fn page_entries(page_index: usize, page_count: usize) -> Vec<Option<usize>> {
    if page_count <= VISIBLE {
        return (0..page_count).map(Some).collect();
    }
    let last = page_count - 1;
    let (start, end) = if page_index <= 3 {
        (0, 4)
    } else if page_index >= last - 3 {
        (last - 4, last)
    } else {
        (page_index - 1, page_index + 1)
    };
    let mut entries = Vec::with_capacity(VISIBLE);
    if start > 0 {
        entries.push(Some(0));
        entries.push(None);
    }
    entries.extend((start..=end).map(Some));
    if end < last {
        entries.push(None);
        entries.push(Some(last));
    }
    entries
}

/// A named navigation region of ordinary independent Buttons.
///
/// The application supplies a zero-based page index and the total page count.
/// A nonempty collection requires `page_index < page_count`; an empty collection
/// uses index zero and displays `No pages`. Numbered buttons mark the current
/// page with native current-page metadata; elided runs are a static
/// `More pages` label. The callback requests another page; the application owns
/// the retained index and its data. No arrow keys or extra Tab stop are added.
#[derive(IntoElement)]
pub(crate) struct Pagination {
    base: ObservedElement<Stateful<Div>>,
    id: ElementId,
    label: SharedString,
    page_index: usize,
    page_count: usize,
    palette: Palette,
    on_change: Option<PageChange>,
}

impl Pagination {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        page_index: usize,
        page_count: usize,
        palette: Palette,
    ) -> Self {
        assert!(
            if page_count == 0 {
                page_index == 0
            } else {
                page_index < page_count
            },
            "page index must identify an available page, or be zero for no pages"
        );
        let id = id.into();
        Self {
            base: div().id(id.clone()).test_support(),
            id,
            label: label.into(),
            page_index,
            page_count,
            palette,
            on_change: None,
        }
    }

    pub(crate) fn on_change(
        mut self,
        on_change: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

fn page_button(
    id: ElementId,
    label: impl Into<SharedString>,
    target: Option<usize>,
    current: bool,
    on_change: Option<PageChange>,
    palette: Palette,
) -> Button {
    Button::new(id)
        .accessibility_label(label)
        .disabled(target.is_none())
        .current(current)
        .selected(current)
        .h(palette.control_height)
        .min_w(palette.control_height)
        .px_3()
        .flex()
        .items_center()
        .justify_center()
        .rounded(palette.radius)
        .border_1()
        .border_color(gpui::transparent_black())
        .text_color(palette.foreground)
        .hover(|style| style.bg(palette.highlight))
        .focus_visible(|style| style.border_color(palette.accent))
        .styles(|styles| {
            styles
                .selected(|style| style.border_color(palette.border).bg(palette.field))
                .disabled(|style| style.opacity(0.45))
        })
        .when_some(
            on_change.zip(target).filter(|_| !current),
            |button, (change, page)| button.on_click(move |_, window, cx| change(page, window, cx)),
        )
}

impl RenderOnce for Pagination {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let palette = self.palette;
        let page_index = self.page_index;
        let previous_page = page_index.checked_sub(1).filter(|_| self.page_count > 0);
        let next_page = Some(page_index + 1).filter(|page| *page < self.page_count);

        let mut children: Vec<AnyElement> = vec![
            page_button(
                (self.id.clone(), "previous").into(),
                "Previous page",
                previous_page,
                false,
                self.on_change.clone(),
                palette,
            )
            .child("Previous")
            .into_any_element(),
        ];
        if self.page_count == 0 {
            children.push(
                div()
                    .id((self.id.clone(), "empty"))
                    .role(Role::Label)
                    .aria_label("No pages")
                    .px_3()
                    .text_color(palette.muted)
                    .child("No pages")
                    .into_any_element(),
            );
        }
        for (position, entry) in page_entries(page_index, self.page_count)
            .into_iter()
            .enumerate()
        {
            children.push(match entry {
                Some(page) => {
                    let label = if page == page_index {
                        format!("Page {}, current page", page + 1)
                    } else {
                        format!("Page {}", page + 1)
                    };
                    page_button(
                        (self.id.clone(), SharedString::from(format!("page-{page}"))).into(),
                        label,
                        Some(page),
                        page == page_index,
                        self.on_change.clone(),
                        palette,
                    )
                    .child((page + 1).to_string())
                    .into_any_element()
                }
                None => div()
                    .id((
                        self.id.clone(),
                        SharedString::from(format!("more-{position}")),
                    ))
                    .role(Role::Label)
                    .aria_label("More pages")
                    .h(palette.control_height)
                    .min_w(palette.control_height)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(palette.muted)
                    .child("…")
                    .into_any_element(),
            });
        }
        children.push(
            page_button(
                (self.id.clone(), "next").into(),
                "Next page",
                next_page,
                false,
                self.on_change,
                palette,
            )
            .child("Next")
            .into_any_element(),
        );

        self.base
            .role(Role::Navigation)
            .aria_label(self.label)
            .flex()
            .flex_wrap()
            .items_center()
            .justify_center()
            .gap_1()
            .text_size(palette.font_size)
            .children(children)
    }
}

#[cfg(test)]
mod tests {
    use super::page_entries;

    #[test]
    fn entries_keep_ends_and_neighbors_with_constant_width() {
        assert_eq!(page_entries(0, 0), []);
        assert_eq!(
            page_entries(2, 5),
            [Some(0), Some(1), Some(2), Some(3), Some(4)]
        );
        let near_start = [Some(0), Some(1), Some(2), Some(3), Some(4), None, Some(9)];
        assert_eq!(page_entries(0, 10), near_start);
        assert_eq!(page_entries(3, 10), near_start);
        assert_eq!(
            page_entries(5, 10),
            [Some(0), None, Some(4), Some(5), Some(6), None, Some(9)]
        );
        let near_end = [Some(0), None, Some(5), Some(6), Some(7), Some(8), Some(9)];
        assert_eq!(page_entries(6, 10), near_end);
        assert_eq!(page_entries(9, 10), near_end);
        for page_count in 8..30 {
            for page_index in 0..page_count {
                let entries = page_entries(page_index, page_count);
                assert_eq!(entries.len(), 7);
                assert!(entries.contains(&Some(page_index)));
                assert_eq!(entries.first(), Some(&Some(0)));
                assert_eq!(entries.last(), Some(&Some(page_count - 1)));
            }
        }
    }
}
