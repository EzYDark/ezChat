#![windows_subsystem = "windows"]

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: "h-screen w-screen flex",
            div { class: "h-full w-14 bg-[color:var(--color-background-contrast)] border border-[color:var(--color-border-muted)]" }
            div { class: "w-50 bg-[color:var(--color-background-contrast)] border-r border-[color:var(--color-border-muted)]",
                div { class: "h-10 w-full flex items-center px-2 bg-[color:var(--color-background-contrast)] border-b border-[color:var(--color-border-muted)]",
                    p { "EzYDark" }
                }
            }
            div { class: "flex-1 bg-[color:var(--color-background)]",
                div { class: "h-10 w-full bg-[color:var(--color-background-contrast)] border-b border-[color:var(--color-border-muted)]" }
            }
            div { class: "w-60 bg-[color:var(--color-background-contrast)] border-l border-[color:var(--color-border-muted)]" }
        }
    }
}
