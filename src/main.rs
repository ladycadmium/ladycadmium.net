use dioxus::prelude::*;

const STYLE_FILE: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE_FILE }
        p {
            "Hello world!"
        }
    }
}
