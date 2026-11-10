#![allow(dead_code)]
use dioxus::prelude::*;

/// The Viewport handles all shared data and state regarding the webpage
#[derive(Clone, Copy)]
pub struct Viewport {
    pub inner_width: Signal<f64>,
    pub inner_height: Signal<f64>,
    pub state: Signal<ViewportState>,
}

/// Run anywhere to fetch information Viewport data
pub fn use_viewport() -> Viewport {
    use_context::<Viewport>()
}

/// Run once in App() to create the Viewport
pub fn use_viewport_provider() -> Viewport {
    let mut inner_width = use_signal(|| 1920.0);
    let mut inner_height = use_signal(|| 1080.0);
    let state = use_signal(|| ViewportState::default());
    let viewport = use_context_provider(|| Viewport { inner_width, inner_height, state });
    
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


/// The visual state of the viewport, ranging from preferences to the current<br>
/// tab. Everything visual that can change is defined here
#[derive(Clone, PartialEq, Debug)]
pub struct ViewportState {
    pub example: usize
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            example: 0
        }
    }
}