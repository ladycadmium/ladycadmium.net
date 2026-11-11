mod common;
mod tabbing;
mod home;
mod blog;
mod games;
mod art;
mod writing;
mod repos;
mod settings;

use crate::{common::use_viewport_provider, tabbing::PageContainer};
use dioxus::prelude::*;

const STYLE_FILE: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _ = use_viewport_provider();
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE_FILE }
        PageContainer{}
    }
}