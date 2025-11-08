#![windows_subsystem = "windows"]

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_free_icons::{
    icons::fi_icons::{FiChevronDown, FiPlus, FiSettings, FiX},
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

// #[component]
// pub fn SecondSidebar() -> Element {
//     let mut state = use_context::<ModalState>();

//     rsx! {
//         // Second Sidebar
//         div { class: "h-full w-54 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] flex flex-col",
//             // Modal Button
//             div {
//                 class: "w-full bg-[var(--color-panel)] border-b border-[var(--color-border-muted)] flex flex-col",
//                 onpointerenter: {
//                     move |_| {
//                         state.is_cursor_in_modal.set(true);
//                     }
//                 },
//                 onpointerleave: {
//                     move |_| {
//                         state.is_cursor_in_modal.set(false);
//                     }
//                 },
//                 button {
//                     class: "h-16 w-full bg-[var(--color-background-contrast)] border-b border-[var(--color-border-muted)] flex items-center justify-between px-3 ",
//                     onclick: {
//                         move |_| {
//                             state.is_modal_open.with_mut(|open| *open = !*open);
//                         }
//                     },
//                     h1 { class: "font-bold text-lg", "EzYDark" }
//                     if *state.is_modal_open.read() {
//                         Icon {
//                             icon: FiX,
//                             class: "w-5 h-5 text-[var(--color-text-muted)]",
//                         }
//                     } else {
//                         Icon {
//                             icon: FiChevronDown,
//                             class: "w-5 h-5 text-[var(--color-text-muted)]",
//                         }
//                     }
//                 }
//                 if *state.is_modal_open.read() {
//                     div { class: "bg-[var(--color-panel)] flex-1 absolute" }
//                 }
//             }
//             div { class: "h-full w-full relative",
//                 if *state.is_modal_open.read() {
//                     // Modal Content
//                     div { class: "h-50 w-full bg-[var(--color-panel)] absolute" }
//                 }
//                 h1 { class: "text-lg font-bold", "EzYDark" }
//             }
//         }
//     }
// }

#[component]
pub fn SecondSidebar() -> Element {
    let mut state = use_context::<ModalState>();

    rsx! {
        // Second Sidebar
        div { class: "h-full w-58 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] flex flex-col",
            // Modal
            button {
                class: "w-full h-16 border border-[var(--color-border-muted)] flex justify-between items-center px-3 cursor-pointer",
                onclick: {
                    move |e| {
                        e.stop_propagation();
                        state.is_modal_open.with_mut(|open| *open = !*open);
                    }
                },
                p { class: "text-lg font-bold", "EzYDark" }
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
            div { class: "h-full w-full relative",
                if *state.is_modal_open.read() {
                    div {
                        class: "w-full h-50 bg-[var(--color-panel)] absolute",
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
                    }
                }
                div { class: "w-full h-full flex flex-col justify-end",
                    h1 { class: "w-full h-20 border border-[var(--color-border-muted)] flex items-center p-3",
                        img {
                            src: asset!("/assets/images/cat.png"),
                            class: "w-12 h-12 object-cover aspect-square",
                        }
                        div { class: "w-full h-full flex justify-between items-center ml-3",
                            div {
                                p { class: "font-bold text-base", "Kheper#1234" }
                                p { class: "text-xs text-[var(--color-text-muted)]",
                                    "Online"
                                }
                            }
                            Icon {
                                icon: FiSettings,
                                class: "w-5 h-5 text-[var(--color-text-muted)] cursor-pointer active:text-[var(--color-text)] transition-all duration-100",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FirstSidebar() -> Element {
    rsx! {
        div { class: "h-full w-20 bg-[var(--color-background-contrast)] border-r border-[var(--color-border-muted)] grid justify-center content-start gap-3 p-3",
            button { class: "h-14 w-14 border-2 border-[var(--color-border)] hover:bg-[var(--color-panel)] cursor-pointer active:bg-[var(--color-panel-active)]",
                Icon {
                    icon: FiPlus,
                    class: "w-full h-full text-[var(--color-text-muted)] p-3 hover:text-[var(--color-text)]",
                }
            }
            button { class: "h-14 w-14 border border-[var(--color-border)] hover:scale-110 transition-all duration-100 cursor-pointer",
                img {
                    src: asset!("/assets/images/cat.png"),
                    class: "w-full h-full object-cover aspect-square",
                }
            }
            button { class: "h-14 w-14 border border-[var(--color-border)] hover:scale-110 transition-all duration-100 cursor-pointer",
                img {
                    src: asset!("/assets/images/cat.png"),
                    class: "w-full h-full object-cover aspect-square",
                }
            }
            button { class: "h-14 w-14 border border-[var(--color-border)] hover:scale-110 transition-all duration-100 cursor-pointer",
                img {
                    src: asset!("/assets/images/cat.png"),
                    class: "w-full h-full object-cover aspect-square",
                }
            }
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
