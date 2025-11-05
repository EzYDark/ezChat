#![windows_subsystem = "windows"]

use std::rc::Rc;

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_free_icons::{icons::fi_icons::FiChevronDown, Icon};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

static LILEX_REGULAR: Asset = asset!(
    "/assets/fonts/Lilex/Lilex-Regular.ttf",
    AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new().with_always_on_top(false)
            )
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    let lilex_regular = format!(
        r#"
            @font-face {{
              font-family: "Lilex";
              src: url("{}") format("truetype");
              font-weight: normal;
              font-style: normal;
            }}
        "#,
        LILEX_REGULAR
    );

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { "{lilex_regular}" }

        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let mut modal_is_open = use_signal(|| false);
    let mut cursor_in_modal = use_signal(|| false);

    let close_modal: Rc<dyn Fn()> = {
        let mut modal_is_open = modal_is_open.clone();
        let mut cursor_in_modal = cursor_in_modal.clone();
        Rc::new(move || {
            modal_is_open.with_mut(|open| {
                if *open {
                    *open = false;
                    cursor_in_modal.with_mut(|inside| *inside = false);
                }
            });
        })
    };

    let toggle_modal: Rc<dyn Fn()> = {
        let mut modal_is_open = modal_is_open.clone();
        let mut cursor_in_modal = cursor_in_modal.clone();
        Rc::new(move || {
            modal_is_open.with_mut(|open| {
                let next = !*open;
                *open = next;
                if !next {
                    cursor_in_modal.with_mut(|inside| *inside = false);
                }
            });
        })
    };

    let highlight_button = modal_is_open() && cursor_in_modal();
    let button_classes = if highlight_button {
        "h-12 w-full flex justify-between items-center px-3 border-b border-[color:var(--color-border-muted)] group bg-[color:var(--color-panel)]"
    } else {
        "h-12 w-full flex justify-between items-center px-3 border-b border-[color:var(--color-border-muted)] group bg-[color:var(--color-background-contrast)] hover:bg-[color:var(--color-panel-active)]"
    };
    let icon_classes = if highlight_button {
        "w-4 h-4 text-[var(--color-text)] stroke-2"
    } else {
        "w-4 h-4 text-[var(--color-text-muted)] stroke-2 group-hover:text-[var(--color-text)]"
    };

    rsx! {
        div {
            class: "h-screen w-screen flex",
            onpointerdown: {
                let close_modal = close_modal.clone();
                move |_| close_modal()
            },
            div { class: "h-full w-14 bg-[color:var(--color-background-contrast)] border border-[color:var(--color-border-muted)]" }
            div { class: "w-50 bg-[color:var(--color-background-contrast)] border-r border-[color:var(--color-border-muted)]",
                button {
                    class: button_classes,
                    onpointerdown: move |evt| evt.stop_propagation(),
                    onclick: {
                        let toggle_modal = toggle_modal.clone();
                        move |evt| {
                            evt.stop_propagation();
                            toggle_modal();
                        }
                    },
                    p { class: "font-bold", "EzYDark" }
                    Icon { icon: FiChevronDown, class: icon_classes }
                }
                div { class: "relative",
                    if modal_is_open() {
                        div {
                            class: "absolute top-0 right-0 w-full h-50 bg-[color:var(--color-panel)]",
                            onpointerdown: move |evt| evt.stop_propagation(),
                            onpointerenter: {
                                let mut cursor_in_modal = cursor_in_modal.clone();
                                move |_| cursor_in_modal.with_mut(|inside| *inside = true)
                            },
                            onpointerleave: {
                                let mut cursor_in_modal = cursor_in_modal.clone();
                                move |_| cursor_in_modal.with_mut(|inside| *inside = false)
                            },
                        }
                    }
                }
            }
            div { class: "flex-1 bg-[color:var(--color-background)]",
                div { class: "h-12 w-full bg-[color:var(--color-background-contrast)] border-b border-[color:var(--color-border-muted)]" }
            }
            div { class: "w-60 bg-[color:var(--color-background-contrast)] border-l border-[color:var(--color-border-muted)]" }
        }
    }
}
