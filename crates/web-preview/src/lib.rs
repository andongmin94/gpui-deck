//! Live documentation previews: GPUI Deck components rendered by GPUI's web
//! backend into a browser canvas for the documentation site.
//!
//! The demos compile on every target so native checks cover them; only the
//! entry point that starts GPUI in the browser is WebAssembly-specific.

pub mod demos;
pub mod gallery;
pub mod theme;

#[allow(dead_code, unused_imports)] // Also compiled as source-owned helpers in native consumer apps.
#[path = "../../cli/presets/create/mod.rs"]
mod preset;

// Render the installable blocks with the same sibling recipes and palette as a consumer.
#[path = "../../../blocks/billing-dashboard/dashboard.rs"]
mod billing_dashboard;
#[path = "../../../blocks/login/login.rs"]
mod login;
#[path = "../../../registries/neobrutal/login.rs"]
#[allow(dead_code)] // Installed palette tokens and themed constructors are also native app APIs.
mod neobrutal;
#[path = "../../../blocks/sidebar-settings/settings.rs"]
mod sidebar_settings;

#[cfg(target_family = "wasm")]
mod block_forms;

#[cfg(target_family = "wasm")]
#[path = "../../base/examples/assets/fonts/mod.rs"]
mod app_fonts;

// Compile the sources installed by the CLI so preview and consumer use the same recipes.
#[path = "../../base/examples/presentation.rs"]
mod presentation;
use presentation::{Appearance, Palette};
#[path = "../../base/examples/recipes/mod.rs"]
#[allow(dead_code, unused_imports)] // Each preview uses a subset of the recipe's optional slots.
mod catalogue_recipes;
#[path = "../../base/examples/recipes/chart.rs"]
#[allow(dead_code)]
mod chart_recipe;

// The CLI exposes Chart alongside the catalogue recipes in the installed module.
mod recipes {
    pub(crate) use super::catalogue_recipes::*;
    pub(crate) use super::chart_recipe::{ChartKind, ChartRecipe};
}

#[cfg(target_family = "wasm")]
mod web {
    use std::rc::Rc;

    use gpui::{App, AppContext as _, Application, Styled as _, WindowOptions};
    use gpui_deck::Root;
    use gpui_web::{WebBackendPreference, WebPlatform};
    use wasm_bindgen::prelude::*;
    use web_sys::{MutationObserver, MutationObserverInit, UrlSearchParams};

    use crate::{app_fonts, demos, gallery, preset, theme::Theme};

    #[wasm_bindgen]
    pub fn start(use_webgl: bool) {
        gpui_web::init_logging();
        let backend = if use_webgl {
            WebBackendPreference::WebGl
        } else {
            WebBackendPreference::Auto
        };
        let platform = WebPlatform::new_with_backend(false, backend);
        Application::with_platform(Rc::new(platform)).run(move |cx: &mut App| {
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
                let create = demo.clone().downcast::<gallery::Gallery>().ok();
                let preset_frame = frame.clone();
                let preset_param = params.get("preset");
                let read_preset = move || {
                    let code = preset_frame
                        .as_ref()
                        .and_then(|el| el.get_attribute("data-preset"));
                    code.as_deref()
                        .or(preset_param.as_deref())
                        .and_then(preset::Preset::parse)
                };
                if let (Some(create), Some(preset)) = (&create, read_preset()) {
                    create.update(cx, |view, cx| view.set_preset(preset, dark, cx));
                }
                crate::block_forms::attach(&demo, window, cx);
                cx.new(|cx| {
                    if let Some(frame) = frame {
                        let app = cx.to_async();
                        let on_change = Closure::<dyn FnMut()>::new(move || {
                            let next = read_theme();
                            app.update(|cx| {
                                if let (Some(create), Some(preset)) = (&create, read_preset()) {
                                    create.update(cx, |view, cx| view.set_preset(preset, dark, cx));
                                }
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
