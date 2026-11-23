//! The pinned GPUI web platform does not export its AccessKit tree on WASM.
//! Give interactive block and registry previews native controls over the same entities.
use gpui::{AnyView, App, Context, Entity, Window};
use gpui_deck::InputState;
use wasm_bindgen::prelude::*;

use crate::{
    demos::neobrutal::{ButtonDemo, InputDemo},
    login::Login,
    neobrutal,
    sidebar_settings::SidebarSettings,
};

#[wasm_bindgen(raw_module = "./block-forms.js")]
extern "C" {
    #[wasm_bindgen(js_name = mountForm)]
    fn mount_form(kind: &str, action: JsValue);
    #[wasm_bindgen(js_name = setValue)]
    fn set_value(id: &str, value: &str);
    #[wasm_bindgen(js_name = setText)]
    fn set_text(id: &str, value: &str);
    #[wasm_bindgen(js_name = updateLogin)]
    fn update_login(
        email: &str,
        password: &str,
        masked: bool,
        email_error: &str,
        password_error: &str,
        validated: bool,
    );
    #[wasm_bindgen(js_name = updateSettings)]
    fn update_settings(
        section: &str,
        name: &str,
        description: &str,
        applied_name: &str,
        compact: bool,
        mentions: bool,
        digest: bool,
        dirty: bool,
        valid: bool,
        status: &str,
    );
}

pub(super) fn attach(view: &AnyView, window: &Window, cx: &mut App) {
    if let Ok(login) = view.clone().downcast::<Login>() {
        mount("login", login, window, cx, sync_login, edit_login);
    } else if let Ok(login) = view.clone().downcast::<neobrutal::Login>() {
        mount(
            "neobrutal-login",
            login,
            window,
            cx,
            |view, cx| {
                sync_credentials(
                    &view.email,
                    &view.password,
                    view.visible_errors(cx),
                    view.validated,
                    cx,
                )
            },
            |view, action, value, window, cx| match action {
                "email" => replace_input(&view.email, value, window, cx),
                "password" => replace_input(&view.password, value, window, cx),
                "submit" => view.submit(window, cx),
                _ => {}
            },
        );
    } else if let Ok(button) = view.clone().downcast::<ButtonDemo>() {
        mount(
            "neobrutal-button",
            button,
            window,
            cx,
            |view, _| set_text("press", &view.label()),
            |view, action, _, _, cx| {
                if action == "press" {
                    view.press(cx);
                }
            },
        );
    } else if let Ok(input) = view.clone().downcast::<InputDemo>() {
        mount(
            "neobrutal-input",
            input,
            window,
            cx,
            |view, cx| set_value("email", &view.email.read(cx).value()),
            |view, action, value, window, cx| {
                if action == "email" {
                    replace_input(&view.email, value, window, cx);
                }
            },
        );
    } else if let Ok(settings) = view.clone().downcast::<SidebarSettings>() {
        mount(
            "settings",
            settings,
            window,
            cx,
            sync_settings,
            edit_settings,
        );
    }
}

fn mount<T: 'static>(
    kind: &str,
    view: Entity<T>,
    window: &Window,
    cx: &mut App,
    sync: fn(&T, &App),
    edit: fn(&mut T, &str, String, &mut Window, &mut Context<T>),
) {
    let weak = view.downgrade();
    let mut app = window.to_async(cx);
    let action = Closure::<dyn FnMut(String, String)>::new(move |action: String, value| {
        let _ = app.update(|window, cx| {
            if let Some(view) = weak.upgrade() {
                view.update(cx, |screen, cx| edit(screen, &action, value, window, cx));
            }
        });
    });
    mount_form(kind, action.into_js_value());
    sync(view.read(cx), cx);
    cx.observe(&view, move |view, cx| sync(view.read(cx), cx))
        .detach();
}

fn sync_login(view: &Login, cx: &App) {
    sync_credentials(
        &view.email,
        &view.password,
        view.visible_errors(cx),
        view.validated,
        cx,
    );
}

fn sync_credentials(
    email: &Entity<InputState>,
    password: &Entity<InputState>,
    [email_error, password_error]: [Option<&str>; 2],
    validated: bool,
    cx: &App,
) {
    let password = password.read(cx);
    update_login(
        &email.read(cx).value(),
        &password.value(),
        password.presentation().is_masked(),
        email_error.unwrap_or_default(),
        password_error.unwrap_or_default(),
        validated,
    );
}

fn edit_login(
    view: &mut Login,
    action: &str,
    value: String,
    window: &mut Window,
    cx: &mut Context<Login>,
) {
    match action {
        "email" => replace_input(&view.email, value, window, cx),
        "password" => replace_input(&view.password, value, window, cx),
        "reveal" => view.toggle_password(window, cx),
        "submit" => view.submit(window, cx),
        _ => {}
    }
}

fn replace_input(input: &Entity<InputState>, value: String, window: &mut Window, cx: &mut App) {
    input.update(cx, |input, cx| input.replace_all(value, window, cx));
}

fn sync_settings(view: &SidebarSettings, cx: &App) {
    let draft = view.draft(cx);
    let section = view.sections.read(cx).selected_item().unwrap().value();
    let preferences = view.preferences;
    update_settings(
        section,
        &draft.name,
        &draft.description,
        &view.applied.name,
        preferences.compact,
        preferences.mentions,
        preferences.digest,
        draft != view.applied,
        !draft.name.trim().is_empty(),
        view.status(&draft),
    );
}

fn edit_settings(
    view: &mut SidebarSettings,
    action: &str,
    value: String,
    window: &mut Window,
    cx: &mut Context<SidebarSettings>,
) {
    match action {
        "section" => {
            view.sections
                .update(cx, |state, cx| state.select_value(value, window, cx));
        }
        "name" | "description" => {
            let field = if action == "name" {
                &view.name
            } else {
                &view.description
            };
            field.update(cx, |input, cx| input.replace_all(value, window, cx));
        }
        "compact" | "mentions" | "digest" => {
            let option = match action {
                "compact" => &mut view.preferences.compact,
                "mentions" => &mut view.preferences.mentions,
                _ => &mut view.preferences.digest,
            };
            *option = value == "true";
            cx.notify();
        }
        "discard" => view.discard(window, cx),
        "apply" => view.apply(cx),
        _ => {}
    }
}
