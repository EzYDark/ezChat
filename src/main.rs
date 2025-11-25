// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use i_slint_backend_winit::WinitWindowAccessor;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_start_window_drag({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let window = ui.window();
            window.with_winit_window(|winit_window| {
                let _ = winit_window.drag_window();
            });
        }
    });

    ui.on_maximize_window({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let window = ui.window();
            window.set_maximized(!window.is_maximized());
        }
    });

    ui.on_minimize_window({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let window = ui.window();
            window.set_minimized(true);
        }
    });

    ui.on_close_window({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.window().hide();
        }
    });

    ui.run()?;

    Ok(())
}
