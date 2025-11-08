use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fi_icons::{FiChevronDown, FiX},
    Icon,
};

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

#[component]
pub fn Modal() -> Element {
    let mut state = use_context::<ModalState>();

    rsx! {
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
    }
}
