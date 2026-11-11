use crate::{art::ArtTab, blog::BlogTab, common::{set_url_param, url, use_viewport, viewport::{PageLayout, SelectedTab}}, games::GamesTab, home::HomeTab, repos::ReposTab, settings::SettingsTab, writing::WritingTab};
use dioxus::prelude::*;

const TAB_ORDER: [SelectedTab; 7] = [
    SelectedTab::Home,
    SelectedTab::Blog,
    SelectedTab::Games,
    SelectedTab::Art,
    SelectedTab::Writing,
    SelectedTab::Repos,
    SelectedTab::Settings
];

#[component]
pub fn PageContainer() -> Element {
    let viewport = use_viewport();
    let selected_tab = viewport.selected_tab.read().clone();
    let current_layout = viewport.layout.read().clone();

    let panel_pos = match current_layout {
        PageLayout::Vertical => "fixed left-0 right-0 bottom-0 top-16 bg-pagebody focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-generic-tab-focus overflow-auto",
        PageLayout::Horizontal => "fixed top-0 right-0 bottom-0 left-24 bg-pagebody focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-generic-tab-focus overflow-auto",
    };

    rsx! {
        TabBar { tab: selected_tab, layout: current_layout.clone() }
        for t in TAB_ORDER {
            div {
                role: "tabpanel",
                id: "panel-{t.id()}",
                aria_labelledby: "tab-{t.id()}",
                tabindex: "0",
                class: if t == selected_tab {
                    "{panel_pos}"
                } else {
                    "{panel_pos} invisible"
                },
                main {
                    match t {
                        SelectedTab::Home => rsx! { HomeTab{} },
                        SelectedTab::Blog => rsx! { BlogTab{} },
                        SelectedTab::Games => rsx! { GamesTab{} },
                        SelectedTab::Art => rsx! { ArtTab{} },
                        SelectedTab::Writing => rsx! { WritingTab{} },
                        SelectedTab::Repos => rsx! { ReposTab{} },
                        SelectedTab::Settings => rsx! { SettingsTab{} },
                    }
                }
            }
        }
    }
}

#[component]
fn TabBar(tab: SelectedTab, layout: PageLayout) -> Element {    
    let viewport = use_viewport();
    
    rsx! {
        div {
            class: match layout {
                PageLayout::Vertical => "fixed top-0 left-0 right-0 h-16 flex flex-row bg-titlebar text-generic-text-light overflow-auto",
                PageLayout::Horizontal => "fixed top-0 left-0 bottom-0 w-24 flex flex-col bg-titlebar text-generic-text-light overflow-auto",
            },
            
            // Logo container
            div {
                aria_hidden: true,
                class: match layout {
                    PageLayout::Vertical => "w-fit h-full flex items-center pl-2 pr-8 border-b-2 border-generic-accent",
                    PageLayout::Horizontal => "w-24 h-fit flex justify-center pt-2 pb-8 border-r-2 border-generic-accent",
                },
                Logo {
                    class: match layout {
                        PageLayout::Vertical => "h-12 w-auto",
                        PageLayout::Horizontal => "w-[4.5rem] h-auto",
                    }
                }
            }

            // Spacer (between logo and buttons)
            div {
                aria_hidden: true,
                class: match layout {
                    PageLayout::Vertical => "w-full h-full border-b-2 border-generic-accent",
                    PageLayout::Horizontal => "w-24 h-full border-r-2 border-generic-accent",
                }
            }

            // Buttons
            div {
                role: "tablist",
                aria_label: "Webpage tabs",
                aria_orientation: match layout {
                    PageLayout::Vertical => "horizontal",
                    PageLayout::Horizontal => "vertical",
                },
                class: match layout {
                    PageLayout::Vertical => "h-full w-fit flex flex-row items-end",
                    PageLayout::Horizontal => "h-fit w-24 flex flex-col items-end",
                },

                // Keyboard navigation
                onkeydown: move |evt| {
                    let tab_count = TAB_ORDER.len();
                    let current = TAB_ORDER.iter().position(|t| *t == tab).unwrap_or(0);
                    let (prev_key, next_key) = match layout {
                        PageLayout::Vertical => (Key::ArrowLeft, Key::ArrowRight),
                        PageLayout::Horizontal => (Key::ArrowUp, Key::ArrowDown),
                    };
                    let key = evt.key();

                    // Ignore modified keys (eg. Ctrl + Left)
                    if evt.modifiers().intersects(Modifiers::ALT | Modifiers::CONTROL | Modifiers::META) {
                        return;
                    }

                    let next = if key == next_key {
                        (current + 1) % tab_count
                    } else if key == prev_key {
                        (current + tab_count - 1) % tab_count
                    } else if key == Key::Home {
                        0
                    } else if key == Key::End {
                        tab_count - 1
                    } else {
                        return;
                    };

                    evt.prevent_default();
                    let target = TAB_ORDER[next];
                    let mut vp = viewport.clone();
                    vp.selected_tab.set(target);
                    set_url_param(url::TAB_PARAM_KEY, target.url_param(), true);
                    let _ = document::eval(&format!(
                        "document.getElementById('tab-{}')?.focus()",
                        target.id()
                    ));
                },

                for t in TAB_ORDER {
                    TabBarButton { selected_tab: tab, label: t, layout: layout.clone() }
                    TabBarButtonSpacer { layout: layout.clone() }
                }
            }
        }
    }
}

#[component]
fn Logo(class: String) -> Element {
    rsx! {
        svg {
            class: "{class}",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 100 61.898",
            path {
                class: "stroke-generic-accent",
                d: "M32.4673 37.8104c-2.90972.13429-3.684 4.06139-3.0287 4.7631 1.08719 1.16417 3.85273-.22224 4.00645-2.41729.2535-3.6198.34333-3.41422.20144-.17906-.11191 2.55158 1.95712 1.55328 2.26494-.1031.36928-1.98715 1.33648-3.35336 2.3935-3.55562 2.58063-.49382 2.49547 2.59865 1.10199 3.8656-1.51203 1.3747-4.6587.51792-2.48107.73066 1.99091.1945 3.19637-1.07025 3.67343-5.74208.44623-4.36994 1.17617-8.61653.50553-3.65884-.70857 5.23794-.48336 9.4676 1.76984 7.33181 1.2746-1.20819 1.76926-5.88312 1.14255-2.481-.6267 3.40212.98855 3.65706 2.60008 2.35889 1.61153-1.29819 1.73536-6.12324 2.00395-3.92977.26858 2.19348-2.0694 16.83516-6.4819 15.51484-1.3648-.40837-1.37662-1.76413-.8157-2.4304m19.42892-17.93741c-2.90972.1343-3.684 4.0614-3.02869 4.76311 1.08719 1.16417 3.85273-.22224 4.00645-2.41729.2535-3.61981.34333-3.41422.20144-.17907-.1119 2.5516 1.93698 1.4868 2.2448-.16959.36928-1.98713 1.33648-3.35335 2.39352-3.55561 2.5806-.49382 2.49545 2.59865 1.10197 3.8656-1.51202 1.37471-4.65869.51793-2.48107.73066 1.99091.1945 3.19638-1.07025 3.67343-5.74208.44621-4.36993 1.17615-8.61653.50551-3.65884-.70855 5.23795.2732 9.88795 1.1767 6.91774.16668-.5479.01309-3.32407.86415-3.75674.85104-.43267 1.6026 4.73217 1.69384 2.81597.07835-1.64511.63038-4.7884 1.2534-3.14342.60864 1.60696 1.34295 3.46797 1.43248 2.68458.30005-2.62554.47003-5.2157 1.16388-3.35796.69387 1.85774 1.58986 3.62468 2.20468 2.18289.3245-.761.09152-4.60752.5244-3.72345.43282.88408.75062 3.43693 1.98276 3.34136 1.40562-.10903.8987-6.46504 1.22508-4.0607.13613 1.00279.57306 3.12994 1.65946 3.02746 1.0594-.09993 1.21716-3.0502 1.42675-4.49143 0 0 .59906 5.504 1.43153 3.14609.83698-2.37066.5863-4.0214 1.19235-3.66906.39182.22778 1.14118 5.23335 1.57764 2.83843.43646-2.39491.37714-4.00258 1.20795-3.392.92888.68266 1.53025 4.49824 1.575 2.8979s.3143-4.20343.89372-3.55716c.47538.53025 1.30791 2.12647 1.70537 3.08587",
                style: "fill:none;stroke-opacity:1;stroke-width:1.76953;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none"
                }
            path {
                class: "stroke-generic-accent",
                d: "M6.63094 19.27344c6.78702-8.5967 18.18685-12.6139 23.89804-7.61003s5.2886 22.60623-18.83962 43.95903c-6.44822 5.7065-15.17526 7.92338-8.28148 1.38771 4.07683-3.86506 9.16614-6.95374 18.23058-8.8898C33.99292 45.48156 79.14992 32.3 99.11502 25.36145",
                style: "fill:none;stroke-opacity:1;stroke-width:1.76953;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none"
            }
            path {
                class: "stroke-generic-accent",
                d: "M81.11957 1.9047C64.8252-8.48074 32.8631 63.59058 75.21062 37.53746",
                style: "fill:none;stroke-opacity:1;stroke-width:1.76953;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none"
            }
            circle {
                class: "fill-generic-accent",
                cx: "79.00899",
                cy: "21.17108",
                r: "1.00523",
                style: "fill-opacity:1;stroke:none;stroke-width:1.76953;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none"
            }
        }
    }
}

#[component]
fn TabBarButton(selected_tab: SelectedTab, label: SelectedTab, layout: PageLayout) -> Element {
    let viewport = use_viewport();

    let button_layout = match layout {
        PageLayout::Vertical => "px-4 pb-2 pt-1 focus-visible:outline-2 focus-visible:outline-generic-tab-focus cursor-pointer",
        PageLayout::Horizontal => "pl-2 pr-4 py-2 focus-visible:outline-2 focus-visible:outline-generic-tab-focus cursor-pointer",
    };
    let button_bold = if selected_tab == label { "font-bold" } else { "" };

    rsx! {
        div {
            role: "presentation",
            class: match layout {
                PageLayout::Vertical => if selected_tab == label {
                    "h-fit w-fit bg-pagebody border-2 border-generic-accent border-b-pagebody rounded-t-[0.25rem]"
                } else {
                    "h-fit w-fit border-2 border-generic-accent rounded-t-[0.25rem]"
                },

                PageLayout::Horizontal => if selected_tab == label {
                    "h-fit w-fit bg-pagebody border-2 border-generic-accent border-r-pagebody rounded-l-[0.25rem]"
                } else {
                    "h-fit w-fit border-2 border-generic-accent rounded-l-[0.25rem]"
                },
            },
            button {
                r#type: "button",
                role: "tab",
                id: "tab-{label.id()}",
                aria_selected: if selected_tab == label { "true" } else { "false" },
                aria_controls: "panel-{label.id()}",
                tabindex: if selected_tab == label { "0" } else { "-1" },
                "width-label": "{label.label()}",
                class: "{button_layout} {button_bold} after:content-[attr(width-label)] after:block after:h-0 after:overflow-hidden after:invisible after:font-bold",
                onclick: move |_| {
                    let mut vp = viewport.clone();
                    set_url_param(url::TAB_PARAM_KEY, label.url_param(), true);
                    vp.selected_tab.set(label.clone());
                },
                {label.label()}
            }
        }
    }
}

#[component]
fn TabBarButtonSpacer(layout: PageLayout) -> Element {
    rsx! {
        div {
            aria_hidden: true,
            class: match layout {
                PageLayout::Vertical => "w-2 h-full border-b-2 border-generic-accent",
                PageLayout::Horizontal => "w-24 h-2 border-r-2 border-generic-accent",
            }
        }
    }
}