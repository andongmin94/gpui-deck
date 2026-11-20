//! Live documentation previews: GPUI Design components rendered by GPUI's web
//! backend into a browser canvas for the documentation site.
//!
//! The demos compile on every target so native checks cover them; only the
//! entry point that starts GPUI in the browser is WebAssembly-specific.

pub mod demos;
pub mod theme;

#[cfg(target_family = "wasm")]
mod web {
    use gpui::{App, AppContext as _, WindowOptions};
    use gpui_design::Root;
    use wasm_bindgen::prelude::*;

    use crate::{demos, theme::Theme};

    /// Reads `?c=<demo>&theme=light|dark` from the frame's address.
    fn query() -> (String, Theme) {
        let search = web_sys::window()
            .and_then(|window| window.location().search().ok())
            .unwrap_or_default();
        let mut name = String::from("showcase");
        let mut theme = Theme::light();
        for pair in search.trim_start_matches('?').split('&') {
            match pair.split_once('=') {
                Some(("c", value)) => name = value.to_owned(),
                Some(("theme", "dark")) => theme = Theme::dark(),
                _ => {}
            }
        }
        (name, theme)
    }

    #[wasm_bindgen(start)]
    pub fn start() {
        gpui_platform::web_init();
        let (name, theme) = query();
        gpui_platform::single_threaded_web().run(move |cx: &mut App| {
            // Browsers expose no system fonts to GPUI; ship Inter (SIL OFL 1.1).
            cx.text_system()
                .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
                    "../assets/Inter-Regular.ttf"
                ))])
                .ok();
            gpui_design::init(cx);
            cx.open_window(WindowOptions::default(), move |window, cx| {
                let demo = demos::open(&name, theme, window, cx);
                cx.new(|cx| Root::new(demo, window, cx))
            })
            .ok();
        });
    }
}
