// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_move_window({
        let ui_handle = ui.as_weak();
        move |dx, dy| {
            let ui = ui_handle.unwrap();
            let window = ui.window();
            let position = window.position();
            let scale_factor = window.scale_factor();
            let dx_px = dx * scale_factor;
            let dy_px = dy * scale_factor;
            window.set_position(slint::PhysicalPosition::new(
                position.x + dx_px as i32,
                position.y + dy_px as i32,
            ));
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

    ui.run()?;

    Ok(())
}
