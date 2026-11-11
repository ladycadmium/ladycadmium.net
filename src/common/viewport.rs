#![allow(dead_code)]
use dioxus::prelude::*;
use web_sys::{wasm_bindgen::{JsCast, closure::Closure}, window};

use crate::common::{clear_url_param, determine_page_layout, get_url_param, set_url_param, url};

/// The Viewport handles all shared data and state regarding the webpage
#[derive(Clone, Copy)]
pub struct Viewport {
    pub inner_width: Signal<f64>,
    pub inner_height: Signal<f64>,
    pub selected_tab: Signal<SelectedTab>,
    pub layout: Signal<PageLayout>
}

/// Run anywhere to fetch information Viewport data
pub fn use_viewport() -> Viewport {
    use_context::<Viewport>()
}

/// Run once in App() to create the Viewport
pub fn use_viewport_provider() -> Viewport {
    let mut inner_width = use_signal(|| 1920.0);
    let mut inner_height = use_signal(|| 1080.0);
    let selected_tab = use_signal(|| SelectedTab::Home);
    let layout = use_signal(|| PageLayout::Vertical);
    let mut viewport = use_context_provider(|| Viewport {
        inner_width,
        inner_height,
        selected_tab,
        layout
    });

    // Update the page layout
    determine_page_layout();

    // Fetch any url parameters
    let selected_tab_from_param = SelectedTab::from_url_param(get_url_param(url::TAB_PARAM_KEY).unwrap_or_default()).unwrap_or(SelectedTab::Home);
    viewport.selected_tab.set(selected_tab_from_param);

    // Event listener for back/forward buttons
    use_hook({
        let mut vp = viewport.clone();
        move || {
            let on_popstate = Closure::wrap(Box::new(move || {
                let selected_tab = SelectedTab::from_url_param(
                    get_url_param(url::TAB_PARAM_KEY).unwrap_or_default()
                ).unwrap_or(SelectedTab::Home);
                vp.selected_tab.set(selected_tab);
            }) as Box<dyn FnMut()>);

            if let Some(win) = window() {
                let _ = win.add_event_listener_with_callback("popstate", on_popstate.as_ref().unchecked_ref());
            }
            on_popstate.forget();
        }
    });

    // Event listener to update inner_width and inner_height signals
    use_future(move || async move {
        let mut js = document::eval(r#"
            // Throttled to one update per frame
            let pending = false;
            const send = () => {
                pending = false;
                dioxus.send([window.innerWidth, window.innerHeight]);
            };

            // Initial value
            send();

            // Event listener
            window.addEventListener('resize', () => {
                if (!pending) {
                    pending = true;
                    requestAnimationFrame(send);
                }
            });
            await new Promise(() => {});
        "#);

        while let Ok((w, h)) = js.recv::<(f64, f64)>().await {
            if *inner_width.peek() != w { inner_width.set(w); }
            if *inner_height.peek() != h { inner_height.set(h); }
        }
    });

    viewport
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectedTab {
    Home,
    Blog,
    Games,
    Art,
    Writing,
    Repos,
    Settings
}

impl SelectedTab {
    pub fn label(&self) -> &'static str {
        match self {
            SelectedTab::Home => "Home",
            SelectedTab::Blog => "Blog",
            SelectedTab::Games => "Games",
            SelectedTab::Art => "Art",
            SelectedTab::Writing => "Writing",
            SelectedTab::Repos => "Repos",
            SelectedTab::Settings => "Settings",
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            SelectedTab::Home => "home",
            SelectedTab::Blog => "blog",
            SelectedTab::Games => "games",
            SelectedTab::Art => "art",
            SelectedTab::Writing => "writing",
            SelectedTab::Repos => "repos",
            SelectedTab::Settings => "settings",
        }
    }

    pub fn url_param(&self) -> &'static str {
        match self {
            SelectedTab::Home => "h",
            SelectedTab::Blog => "b",
            SelectedTab::Games => "g",
            SelectedTab::Art => "a",
            SelectedTab::Writing => "w",
            SelectedTab::Repos => "r",
            SelectedTab::Settings => "s",
        }
    }

    pub fn from_url_param(s: String) -> Option<Self> {
        match s.as_str() {
            "h" => Some(SelectedTab::Home),
            "b" => Some(SelectedTab::Blog),
            "g" => Some(SelectedTab::Games),
            "a" => Some(SelectedTab::Art),
            "w" => Some(SelectedTab::Writing),
            "r" => Some(SelectedTab::Repos),
            "s" => Some(SelectedTab::Settings),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PageLayout {
    Vertical,
    Horizontal
}