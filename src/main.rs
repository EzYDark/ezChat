#![windows_subsystem = "windows"]

use std::time::Duration;

use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_free_icons::{
    Icon,
    icons::{
        fi_icons::{FiChevronDown, FiHash, FiHeadphones, FiMic, FiPlus, FiSend, FiSettings, FiX},
        ld_icons::{LdCalendar, LdNotebook},
    },
};
use log::info;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;

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
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info),
        );
    }

    #[cfg(not(target_os = "android"))]
    {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

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
            // Chat messages
            div {
                class: "flex-1 m-3 overflow-y-auto custom-scrollbar",
                div {
                    class: "flex flex-col justify-end gap-3 min-h-full",
                    for x in 0..100 {
                        div {
                            class: "flex flex-col gap-1",
                            class: if x % 2 == 0 { "items-end" },
                            p { class: "text-xs text-[var(--color-text-muted)]",
                                "EzYDark"
                            }
                            div {
                                class: "w-fit min-w-[100px] max-w-[60%] min-h-20 bg-[var(--color-panel)] p-2",
                                p { "Hello. How are you? We are making a new chat app. It's going to be awesome. I'm so excited to see how it turns out. We are not testing this message. We are just testing the layout." }
                            }
                            p { class: "text-xs text-[var(--color-text-muted)]",
                                "12:00"
                            }
                        }
                        div {
                            class: "flex flex-col gap-1",
                            class: if x % 3 == 0 { "items-end" },
                            p { class: "text-xs text-[var(--color-text-muted)]",
                                "EzYDark"
                            }
                            div {
                                class: "w-fit min-w-[275px] max-w-[60%] min-h-20 bg-[var(--color-panel)] border border-[var(--color-border-muted)] p-2",
                                p { "Hello. How are you? We are making a new chat app. It's going to be awesome. I'm so excited to see how it turns out. We are not testing this message. We are just testing the layout." }
                                div {
                                    class: "flex",
                                    class: if x % 3 == 0 { "justify-end" },
                                    div {
                                        class: "grid grid-cols-2 gap-1 mt-3 max-w-[400px]",
                                        img {
                                            src: asset!("/assets/images/cat.png"),
                                            class: "object-cover aspect-square cursor-pointer",
                                        }
                                        img {
                                            src: asset!("/assets/images/screenshot_test.png"),
                                            class: "object-cover aspect-square cursor-pointer",
                                        }
                                        if x % 2 == 0 {
                                            img {
                                                src: asset!("/assets/images/screenshot_test2.png"),
                                                class: "object-cover aspect-square cursor-pointer",
                                            }
                                            img {
                                                src: asset!("/assets/images/cat.png"),
                                                class: " object-cover aspect-square cursor-pointer",
                                            }
                                        }
                                    }
                                }
                            }
                            p { class: "text-xs text-[var(--color-text-muted)]",
                                "12:00"
                            }
                        }
                    }
                }
            }
            div {
                class: "flex items-center gap-2 m-3",
                button {
                    class: "w-10 h-10 border border-[var(--color-border-muted)] hover:bg-[var(--color-panel)] cursor-pointer active:bg-[var(--color-panel-active)] flex items-center justify-center",
                    Icon {
                        icon: FiPlus,
                        class: "w-6 h-6 text-[var(--color-text-muted)] cursor-pointer hover:text-[var(--color-text)]",
                    }
                }
                textarea {
                    placeholder: "Type your message...",
                    class: "custom-scrollbar field-sizing-content w-full min-h-10 max-h-50 p-3 border border-[var(--color-border-muted)] focus:outline-none focus:ring-1 focus:ring-[var(--color-accent)] resize-none",
                }
                button {
                    class: "w-10 h-10 border border-[var(--color-border-muted)] hover:bg-[var(--color-panel)] cursor-pointer active:bg-[var(--color-panel-active)] flex items-center justify-center",
                    Icon {
                        icon: FiSend,
                        class: "w-4 h-4 text-[var(--color-text-muted)] cursor-pointer hover:text-[var(--color-text)]",
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    status: String,
}

#[component]
pub fn Main() -> Element {
    let mut modal_state = ModalState::new();
    use_context_provider(|| modal_state);

    // use_hook(|| {
    //     spawn_forever(async move {
    //         let db = match Surreal::new::<RocksDb>("./my_database.db").await {
    //             Ok(db) => {
    //                 println!("Database created successfully");
    //                 db
    //             }
    //             Err(e) => {
    //                 println!("Failed to create database: {}", e);
    //                 return;
    //             }
    //         };

    //         // Select the namespace and database to use
    //         db.use_ns("my_namespace")
    //             .use_db("my_database")
    //             .await
    //             .unwrap();

    //         // Create a new person
    //         let created: Option<Person> = db
    //             .create("person")
    //             .content(Person {
    //                 name: "David".to_string(),
    //                 status: "persistent".to_string(),
    //             })
    //             .await
    //             .unwrap();

    //         println!("Created person: {:?}", created);

    //         // Select all people
    //         let people: Vec<Person> = db.select("person").await.unwrap();
    //         println!("Found people: {:?}", people);
    //     })
    // });

    tokio::spawn(async move {
        let db = match Surreal::new::<Mem>(()).await {
            Ok(db) => {
                info!("Database created successfully");
                db
            }
            Err(e) => {
                error!("Failed to create database: {}", e);
                return;
            }
        };

        // Select the namespace and database to use
        match db.use_ns("my_namespace").use_db("my_database").await {
            Ok(_) => {
                info!("Namespace and database selected successfully");
            }
            Err(e) => {
                error!("Failed to select namespace and database: {}", e);
                return;
            }
        }

        // Create a new person
        let created: Option<Person> = match db
            .create("person")
            .content(Person {
                name: "David".to_string(),
                status: "persistent".to_string(),
            })
            .await
        {
            Ok(created) => created,
            Err(e) => {
                error!("Failed to create person: {}", e);
                return;
            }
        };

        info!("Created person: {:?}", created);

        // Select all people
        let people: Vec<Person> = db.select("person").await.unwrap();
        info!("Found people: {:?}", people);

        loop {
            info!("Background task is running! 2");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

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
