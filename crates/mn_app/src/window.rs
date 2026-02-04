use bevy::prelude::*;

use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode, WindowPosition};
use mn_core::AppWindowCommand;


pub fn windows_control_system(
    mut reader: MessageReader<AppWindowCommand>, // project-local message reader
    mut app_exit_events: MessageWriter<AppExit>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // get the primary window (Single param alternative works too, but this mirrors your code)
    let mut window = match window_query.single_mut() {
        Ok(w) => w,
        Err(_) => return,
    };

    for cmd in reader.read() {
        match cmd {
            AppWindowCommand::Shutdown => {
                app_exit_events.write(AppExit::Success);
            }
            AppWindowCommand::ToggleMaximize => {
                if window.mode != WindowMode::Windowed {
                    window.mode = WindowMode::Windowed;
                } else {
                    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
                }
            }
            AppWindowCommand::Minimize => {
                window.set_minimized(true);
            }
            AppWindowCommand::StartMove => {
                window.start_drag_move();
            }
            AppWindowCommand::StartResize(octant) => {
                window.mode = WindowMode::Windowed;
                window.start_drag_resize(*octant);
            }
        }
    }
}
