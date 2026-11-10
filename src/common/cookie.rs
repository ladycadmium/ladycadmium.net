use web_sys::wasm_bindgen::JsCast;

pub fn get_cookie(key: &str) -> Option<String> {
    html_document()?
        .cookie()
        .ok()?
        .split(';')
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
}

pub fn set_cookie(key: &str, value: &str) {
    if let Some(document) = html_document() {
        let _ = document.set_cookie(&format!(
            "{key}={value}; path=/; max-age=31536000; SameSite=Lax"
        ));
    }
}

pub fn delete_cookie(key: &str) {
    if let Some(document) = html_document() {
        let _ = document.set_cookie(&format!("{key}=; path=/; max-age=0"));
    }
}

fn html_document() -> Option<web_sys::HtmlDocument> {
    web_sys::window()?
        .document()?
        .dyn_into::<web_sys::HtmlDocument>()
        .ok()
}