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
        div { class: "flex items-center justify-center h-screen",
            div { class: "flex flex-col items-center justify-center",
                h1 { class: "text-4xl font-bold text-[color:var(--color-background-contrast)]",
                    "EzChat"
                }
                p { class: "text-lg", "A simple chat application" }
            }
        }
    }
}
