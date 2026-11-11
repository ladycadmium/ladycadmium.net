use crate::common::{use_viewport, viewport::PageLayout};
use dioxus::prelude::*;

pub fn determine_page_layout() {
    let mut viewport = use_viewport();
    let w = viewport.inner_width.read().clone();
    let h = viewport.inner_height.read().clone();
    let aspect = w / h;


    if (w >= 900.0 && h > 256.0) || aspect < 1.2 {
        viewport.layout.set(PageLayout::Vertical);
        web_sys::console::log_1(&format!("Page size changed to [{:.0} x {:.0}]. Updated page layout to 'Vertical'", w, h).into());
    } else {
        viewport.layout.set(PageLayout::Horizontal);
        web_sys::console::log_1(&format!("Page size changed to [{:.0} x {:.0}]. Updated page layout to 'Horizontal'", w, h).into());
    }
}