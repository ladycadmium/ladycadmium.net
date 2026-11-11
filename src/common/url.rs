use dioxus::prelude::*;
use web_sys::{UrlSearchParams, wasm_bindgen::{JsCast, JsValue, closure::Closure}, window};

pub const TAB_PARAM_KEY: &str = "t";

pub fn get_url_param(key: &str) -> Option<String> {
    search_params()?.get(key)
}

pub fn set_url_param(key: &str, value: &str, push: bool) {
    update(key, Some(value), push);
}

pub fn clear_url_param(key: &str, push: bool) {
    update(key, None, push)
}

pub fn clear_url_params(push: bool) {
    let Some(win) = window() else { return };
    let loc = win.location();
    let new_url = format!(
        "{}{}",
        loc.pathname().unwrap_or_default(),
        loc.hash().unwrap_or_default(),
    );
    change_url(&win, &new_url, push);
}

fn search_params() -> Option<UrlSearchParams> {
    let search = window()?.location().search().ok()?;
    UrlSearchParams::new_with_str(&search).ok()
}

fn update(key: &str, value: Option<&str>, push: bool) {
    let Some(win) = window() else { return };
    let Some(params) = search_params() else { return };
    let loc = win.location();

    match value {
        Some(v) => params.set(key, v),
        None => params.delete(key),
    }

    let query = String::from(params.to_string());
    let path = loc.pathname().unwrap_or_default();
    let hash = loc.hash().unwrap_or_default();
    let new_url = if query.is_empty() {
        format!("{path}{hash}")
    } else {
        format!("{path}?{query}{hash}")
    };

    let current = format!("{path}{}{hash}", loc.search().unwrap_or_default());
    if new_url == current {
        return;
    }

    change_url(&win, &new_url, push);
}

fn change_url(win: &web_sys::Window, new_url: &str, push: bool) {
    let loc = win.location();
 
    let current = format!(
        "{}{}{}",
        loc.pathname().unwrap_or_default(),
        loc.search().unwrap_or_default(),
        loc.hash().unwrap_or_default(),
    );
    if new_url == current {
        return;
    }
 
    let Ok(history) = win.history() else { return };
    let _ = if push {
        history.push_state_with_url(&JsValue::NULL, "", Some(new_url))
    } else {
        history.replace_state_with_url(&JsValue::NULL, "", Some(new_url))
    };
}
