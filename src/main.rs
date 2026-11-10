mod common;
use crate::common::use_viewport_provider;
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
        p {
            "Hello world!"
        }
    }
}