//! Search and sort the post archive, then open a full article.
mod articles;
mod blog_view;
mod button;
mod input;
mod layout;
mod palette;
use super::Palette;
pub(crate) use blog_view::Blog;
pub(crate) fn preview(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyView {
    use gpui::AppContext as _;
    cx.new(|cx| Blog::new(palette::air(), None, window, cx))
        .into()
}
