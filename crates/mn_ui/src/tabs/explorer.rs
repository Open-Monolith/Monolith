use crate::theme::ThemeResource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui::{self};
use mn_core::{MonoTab, icons::Icon};

use crate::widgets::sidebar_panel::sidebar_panel;

pub fn show(
    ui: &mut egui::Ui,
    tab: &mut MonoTab,
    icon_textures: &HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>,
    theme: &ThemeResource,
) {
    let my_icons: [Icon; 5] = [
        Icon::TabExplorerAssets,
        Icon::TabExplorerViews,
        Icon::TabExplorerSheets,
        Icon::TabExplorerSchedules,
        Icon::TabExplorerGroups,
    ];

    sidebar_panel(
        ui,
        tab.id,
        icon_textures,
        theme,
        &my_icons,
        "TabExplorer",
        Icon::TabExplorerAssets,
        |ui, selected_icon| {
            match selected_icon {
                Icon::TabExplorerAssets => {
                    ui.label("Assets");
                }
                Icon::TabExplorerGroups => {
                    ui.label("Groups");
                }
                Icon::TabExplorerSchedules => {
                    ui.label("Schedules");
                }
                Icon::TabExplorerSheets => {
                    ui.label("Sheets");
                }
                Icon::TabExplorerViews => {
                    ui.label("Views");
                }
                _ => {
                    ui.label(format!("Not implemented: {:?}", selected_icon));
                }
            }
        },
    );
}
