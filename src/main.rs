#![windows_subsystem = "windows"]

use std::rc::Rc;

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_free_icons::{
    icons::fi_icons::{FiChevronDown, FiX},
    Icon,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

static LILEX_REGULAR: Asset = asset!("/assets/fonts/Lilex/Lilex-Regular.ttf");

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
    let mut isModalOpen = use_signal(|| false);
    let mut isCursorInModal = use_signal(|| false);

    fn close_modal(modal: &mut Signal<bool>) {
        modal.set(false);
    }

    rsx! {
        div {
            class: "h-screen w-screen flex",
            onclick: {
                move |_| {
                    if *isModalOpen.read() && !*isCursorInModal.read() {
                        close_modal(&mut isModalOpen);
                    }
                }
            },
            div { class: "h-full w-16 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)]" }
            div { class: "h-full w-54 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] flex flex-col",
                button {
                    class: "h-14 w-full bg-[var(--color-background-contrast)] border-b border-[var(--color-border-muted)] flex items-center justify-between px-3 hover:bg-[var(--color-panel)] active:bg-[var(--color-panel-active)]",
                    class: if *isModalOpen.read() { "bg-[var(--color-panel)]" },
                    onclick: {
                        move |ev| {
                            ev.stop_propagation();
                            isModalOpen.with_mut(|modal| *modal = !*modal);
                        }
                    },
                    h1 { class: "font-bold", "EzYDark" }
                    if *isModalOpen.read() {
                        Icon {
                            icon: FiX,
                            class: "w-5 h-5 text-[var(--color-text-muted)]",
                        }
                    } else {
                        Icon {
                            icon: FiChevronDown,
                            class: "w-5 h-5 text-[var(--color-text-muted)]",
                        }
                    }
                }
                div { class: "relative",
                    if *isModalOpen.read() {
                        div {
                            class: "absolute top-0 right-0 w-full h-50 bg-[var(--color-panel)]",
                            onpointerenter: {
                                move |_| {
                                    isCursorInModal.set(true);
                                }
                            },
                            onpointerleave: {
                                move |_| {
                                    isCursorInModal.set(false);
                                }
                            },
                        }
                    }
                }
            }
            div { class: "flex-1" }
            div { class: "h-full w-50 bg-[var(--color-background-contrast)] border-l border-[var(--color-border-muted)]" }
        }
    }
}
