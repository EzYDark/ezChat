#![windows_subsystem = "windows"]

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

#[derive(Clone, Copy, PartialEq)]
pub struct ModalState {
    pub is_modal_open: Signal<bool>,
    pub is_cursor_in_modal: Signal<bool>,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            is_modal_open: use_signal(|| false),
            is_cursor_in_modal: use_signal(|| false),
        }
    }
}

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

        Main {}
    }
}

#[component]
pub fn SecondSidebar() -> Element {
    let mut state = use_context::<ModalState>();

    rsx! {
        div { class: "h-full w-54 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] flex flex-col",
            div {
                class: "w-full bg-[var(--color-panel)] border-b border-[var(--color-border-muted)] flex flex-col relative",
                onpointerenter: {
                    move |_| {
                        state.is_cursor_in_modal.set(true);
                    }
                },
                onpointerleave: {
                    move |_| {
                        state.is_cursor_in_modal.set(false);
                    }
                },
                button {
                    class: "h-16 w-full bg-[var(--color-background-contrast)] border-b border-[var(--color-border-muted)] flex items-center justify-between px-3 ",
                    onclick: {
                        move |_| {
                            state.is_modal_open.with_mut(|open| *open = !*open);
                        }
                    },
                    h1 { class: "font-bold text-lg", "EzYDark" }
                    if *state.is_modal_open.read() {
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
                if *state.is_modal_open.read() {
                    div { class: "bg-[var(--color-panel)] flex-1 absolute" }
                }
            }
            div { class: "h-full w-full relative",
                if *state.is_modal_open.read() {
                    div { class: "h-50 w-full bg-[var(--color-panel)] absolute" }
                }
            }
        
        }
    }
}

#[component]
pub fn FirstSidebar() -> Element {
    rsx! {
        div { class: "h-full w-20 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] grid justify-center content-start gap-3 p-3",
            div { class: "h-14 w-14 bg-amber-300 " }
            div { class: "h-14 w-14 bg-amber-300 " }
            div { class: "h-14 w-14 bg-amber-300 " }
        }
    }
}

#[component]
pub fn Main() -> Element {
    let mut modal_state = ModalState::new();
    use_context_provider(|| modal_state);

    rsx! {
        div {
            class: "h-screen w-screen flex",
            onclick: {
                move |_| {
                    if *modal_state.is_modal_open.read()
                        && !*modal_state.is_cursor_in_modal.read()
                    {
                        modal_state.is_modal_open.set(false);
                    }
                }
            },
            FirstSidebar {}
            SecondSidebar {}
            div { class: "flex-1" }
            div { class: "h-full w-50 bg-[var(--color-background-contrast)] border-l border-[var(--color-border-muted)]" }
        }
    }
}
