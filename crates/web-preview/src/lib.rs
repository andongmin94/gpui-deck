//! Live documentation previews: GPUI Deck components rendered by GPUI's web
//! backend into a browser canvas for the documentation site.
//!
//! The demos compile on every target so native checks cover them; only the
//! entry point that starts GPUI in the browser is WebAssembly-specific.

pub mod demos;
pub mod theme;

#[cfg(target_family = "wasm")]
#[path = "../../base/examples/assets/fonts/mod.rs"]
mod app_fonts;

// Compile the sources installed by the CLI so preview and consumer use the same recipes.
#[path = "../../base/examples/presentation.rs"]
mod presentation;
use presentation::{Appearance, Palette};
#[path = "../../base/examples/recipes/mod.rs"]
#[allow(dead_code, unused_imports)] // Each preview uses a subset of the recipe's optional slots.
mod recipes;

#[cfg(target_family = "wasm")]
mod web {
    use gpui::{App, AppContext as _, Styled as _, WindowOptions};
    use gpui_deck::Root;
    use wasm_bindgen::prelude::*;
    use web_sys::{MutationObserver, MutationObserverInit, UrlSearchParams};

    use crate::{app_fonts, demos, theme::Theme};

    #[wasm_bindgen(start)]
    pub fn start() {
        gpui_platform::web_init();
        gpui_platform::single_threaded_web().run(move |cx: &mut App| {
            app_fonts::register(cx);
            gpui_deck::init(cx);
            cx.open_window(WindowOptions::default(), move |window, cx| {
                let browser = web_sys::window().expect("browser preview window");
                let params =
                    UrlSearchParams::new_with_str(&browser.location().search().unwrap_or_default())
                        .expect("read preview parameters");
                let name = params
                    .get("c")
                    .unwrap_or_else(|| "invoice-workspace".into());
                let dark = params.get("theme").as_deref() == Some("dark");
                let fallback = params.get("accent");
                let frame = browser.frame_element().ok().flatten();
                let target = frame.clone();
                let read_theme = move || {
                    let selected = target
                        .as_ref()
                        .and_then(|el| el.get_attribute("data-accent"));
                    Theme::from_site(dark, selected.as_deref().or(fallback.as_deref()))
                };
                let theme = read_theme();
                cx.set_global(theme);
                let demo = demos::open(&name, theme, window, cx);
                cx.new(|cx| {
                    if let Some(frame) = frame {
                        let app = cx.to_async();
                        let on_change = Closure::<dyn FnMut()>::new(move || {
                            let next = read_theme();
                            app.update(|cx| {
                                if cx.global::<Theme>().primary != next.primary {
                                    cx.set_global(next);
                                    cx.refresh_windows();
                                }
                            });
                        });
                        let observer = MutationObserver::new(on_change.as_ref().unchecked_ref())
                            .expect("create preview accent observer");
                        let options = MutationObserverInit::new();
                        options.set_attributes(true);
                        observer
                            .observe_with_options(&frame, &options)
                            .expect("observe preview frame");
                        cx.on_release(move |_, _| {
                            observer.disconnect();
                            drop(on_change);
                        })
                        .detach();
                    }
                    Root::new(demo, window, cx).font_family(app_fonts::FAMILY)
                })
            })
            .ok();
        });
    }
}
