#![windows_subsystem = "windows"]

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_free_icons::{
    icons::{
        fi_icons::{FiChevronDown, FiHash, FiHeadphones, FiMic, FiPlus, FiSend, FiSettings, FiX},
        ld_icons::{LdCalendar, LdHash, LdNotebook, LdUser},
    },
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
        // Second Sidebar
        div { class: "h-full w-76 bg-[var(--color-background-contrast)] border-r border-[var(--color-border)] flex flex-col",
            // Modal
            button {
                class: "w-full h-16 border-b border-[var(--color-border)] flex justify-between items-center px-3 cursor-pointer",
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
                div { class: "w-full h-full flex flex-col",
                    div { class: "flex-1 flex py-1 flex-col",
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: LdCalendar,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Calendar" }
                        }
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: LdNotebook,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Notes" }
                        }
                        div { class: "w-11/12 mx-auto h-px border border-[var(--color-border)]" }
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: FiHash,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Channel 1" }
                        }
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: FiHash,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Channel 2" }
                        }
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: FiHash,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Channel 3" }
                        }
                        div { class: "p-4 h-10 w-full flex gap-2 hover:bg-[var(--color-panel-active)] cursor-pointer items-center",
                            Icon {
                                icon: FiHash,
                                class: "w-5 h-5 text-[var(--color-text-muted)]",
                            }
                            p { "Channel 4" }
                        }
                    }
                    div { class: "w-full h-20 border-t border-[var(--color-border)] flex items-center p-3",
                        img {
                            src: asset!("/assets/images/cat.png"),
                            class: "w-12 h-12 object-cover aspect-square",
                        }
                        div { class: "w-full h-full flex justify-between items-center ml-3",
                            div {
                                div { class: "flex gap-1 items-center",
                                    p { class: "text-base", "Kheper" }
                                    p { class: "text-xs text-[var(--color-text-muted)]",
                                        "#1234"
                                    }
                                }
                                p { class: "text-xs text-[var(--color-text-muted)]",
                                    "Online"
                                }
                            }
                            div { class: "flex gap-2 items-center",
                                Icon {
                                    icon: FiHeadphones,
                                    class: "w-5 h-5 text-[var(--color-text-muted)] cursor-pointer active:text-[var(--color-text)] transition-all duration-100",
                                }
                                Icon {
                                    icon: FiMic,
                                    class: "w-5 h-5 text-[var(--color-text-muted)] cursor-pointer active:text-[var(--color-text)] transition-all duration-100",
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
}

#[component]
pub fn FirstSidebar() -> Element {
    rsx! {
        div { class: "h-full w-20 bg-[var(--color-background-contrast)] border-r border-[var(--color-border)] grid justify-center content-start gap-3 p-3",
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
pub fn ThirdSidebar() -> Element {
    rsx! {
        // Third Sidebar
        div { class: "h-full w-70 bg-[var(--color-background-contrast)] border-l border-[var(--color-border)] p-3 flex flex-col",
            input {
                r#type: "text",
                placeholder: "Search",
                class: "w-full h-10 px-3 border border-[var(--color-border)] focus:outline-none focus:ring-1 focus:ring-[var(--color-accent)]",
            }
            div { class: "w-11/12 mx-auto h-px bg-[var(--color-border)] my-3" }
            div { class: "flex-1 flex flex-col gap-2 mt-2",
                p { class: "text-xs text-[var(--color-text-muted)]", "Members" }
                div { class: "flex gap-2 items-center",
                    img {
                        src: asset!("/assets/images/cat.png"),
                        class: "w-9 h-9 object-cover aspect-square",
                    }
                    p { "Kheper" }
                }
                div { class: "flex gap-2 items-center",
                    img {
                        src: asset!("/assets/images/cat.png"),
                        class: "w-9 h-9 object-cover aspect-square",
                    }
                    p { "EzYDark" }
                }
            }
        }
    }
}

#[component]
pub fn MainChat() -> Element {
    rsx! {
        div {
            class: "flex-1 flex flex-col",
            div {
                class: "flex-1",
            }
            div {
                class: "flex items-end gap-2 m-3",
                button {
                    class: "w-10 h-10 mb-1 border border-[var(--color-border-muted)] hover:bg-[var(--color-panel)] cursor-pointer active:bg-[var(--color-panel-active)] flex items-center justify-center",
                    Icon {
                        icon: FiPlus,
                        class: "w-6 h-6 text-[var(--color-text-muted)] cursor-pointer hover:text-[var(--color-text)]",
                    }
                }
                textarea {
                    placeholder: "Type your message...",
                    class: "field-sizing-content w-full min-h-10 p-3 border border-[var(--color-border-muted)] focus:outline-none focus:ring-1 focus:ring-[var(--color-accent)] resize-none",
                }
                button {
                    class: "w-10 h-10 mb-1 border border-[var(--color-border-muted)] hover:bg-[var(--color-panel)] cursor-pointer active:bg-[var(--color-panel-active)] flex items-center justify-center",
                    Icon {
                        icon: FiSend,
                        class: "w-4 h-4 text-[var(--color-text-muted)] cursor-pointer hover:text-[var(--color-text)]",
                    }
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
            class: "h-screen w-screen flex overflow-hidden",
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
            MainChat {}
            ThirdSidebar {}
        }
    }
}
