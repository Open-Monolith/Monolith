use bevy::prelude::*;
use bevy_egui::egui;
use bevy::platform::collections::HashMap;

use mn_core::{AppWindowCommand, icons::Icon};
use bevy_egui::egui::{Button, vec2};

pub(crate) fn menu_bar(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    mut appwindow_writer: MessageWriter<AppWindowCommand>,
    icon_textures: HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>
) -> egui::InnerResponse<()> {
    egui::MenuBar::new().ui(ui, |ui| {


        let drag_interaction = ui.interact(
            ui.max_rect(), 
            ui.id().with("window_drag_handle"), // Unique ID for this interactor
            egui::Sense::drag()
        );

        // If a drag starts here (and wasn't captured by a button above), send the command.
        if drag_interaction.drag_started() {
            appwindow_writer.write(AppWindowCommand::StartMove);
        }

        file_menu(ctx, ui);
        edit_menu(ctx, ui);
        window_menu(ctx, ui);
        view_menu(ctx, ui);

        ui.add_space(25.);

        workspace_buttons(ui);




        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            window_controls(ctx, ui, &mut appwindow_writer, icon_textures);
            // search_bar(app, ui);
            // topbar_resize(ctx, ui);
        });


    })
}

fn file_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("New Project").clicked() {}
        if ui.button("Open Project...").clicked() {}
        ui.menu_button("Open Recent", |ui| {
            if ui.button("Kahuina.monolith").clicked() {}
            if ui.button("Uptown_mall.monolith").clicked() {}
            if ui.button("Launiu Project.monolith").clicked() {}
        });
        if ui.button("Save").clicked() {}
        if ui.button("Save as...").clicked() {}
        ui.separator();
        ui.menu_button("Import", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("glTF / GLB").clicked() {}
            if ui.button("CAD (.dxf)").clicked() {}
        });
        ui.menu_button("Export", |ui| {
            if ui.button("IFC (.ifc)").clicked() {}
            if ui.button("Sheet (.pdf)").clicked() {}
        });
        ui.separator();
        if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

fn edit_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Edit", |ui| {
        if ui.button("Undo (Ctrl + Z)").clicked() {}
        if ui.button("Redo (Ctrl + Y)").clicked() {}
        ui.separator();
        if ui.button("Cut (Ctrl + X)").clicked() {}
        if ui.button("Copy (Ctrl + C)").clicked() {}
        if ui.button("Paste (Ctrl + V)").clicked() {}
        if ui.button("Delete (Del)").clicked() {}
        ui.separator();
        ui.menu_button("Select All...", |ui| {
            if ui.button("in Current View").clicked() {}
            if ui.button("in Entire Project").clicked() {}
        });
        if ui.button("Deselect All (Alt + A)").clicked() {}
        ui.separator();
        if ui.button("Commands (Space)").clicked() {}
        if ui.button("Preferences...").clicked() {}
    });
}

fn window_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("Window", |ui| {
        ui.menu_button("Workspaces", |ui| {
            if ui.button("Modeling").clicked() {}
            if ui.button("Drafting").clicked() {}
            if ui.button("Forging").clicked() {}
            if ui.button("Rendering").clicked() {}
            if ui.button("Scripting").clicked() {}
            if ui.button("Collaboration").clicked() {}
        });
        ui.separator();
        if ui.button("Scene View").clicked() {}
        if ui.button("Properties").clicked() {}
        if ui.button("File Explorer").clicked() {}
        if ui.button("Scripting").clicked() {}
        if ui.button("Asset Browser").clicked() {}
        if ui.button("Console").clicked() {}
        ui.separator();
        if ui.button("Toggle Fullscreen (F11)").clicked() {}
        if ui.button("Reset Layout").clicked() {}
    });
}

fn view_menu(_ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.menu_button("View", |ui| {
        if ui.button("Documentation").clicked() {}
        if ui.button("Keyboard Shortcuts").clicked() {}
        if ui.button("Report a Bug").clicked() {}
        if ui.button("Community").clicked() {}
        ui.separator();
        if ui.button("Developer Tools").clicked() {}
        if ui.button("About Monolith").clicked() {}
    });
}

fn workspace_buttons(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        if ui.button("Modeling").clicked() {}
        if ui.button("Drafting").clicked() {}
        if ui.button("Forging").clicked() {}
        if ui.button("Rendering").clicked() {}
        if ui.button("Scripting").clicked() {}
        if ui.button("Collaboration").clicked() {}
    });
}

fn window_controls(
    _ctx: &egui::Context,
    ui: &mut egui::Ui,
    appwindow_writer: &mut MessageWriter<AppWindowCommand>,
    icon_textures: HashMap<mn_core::icons::Icon, bevy_egui::egui::TextureId>
) {
    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding = egui::vec2(3.0, 3.0); 
        let image_size = egui::vec2(12.0, 12.0);
        
        // Close
        if ui.add(egui::Button::image((icon_textures[&Icon::WindowClose], image_size))).clicked() {
            appwindow_writer.write(AppWindowCommand::Shutdown);
        }

        // Maximize
        if ui.add(egui::Button::image((icon_textures[&Icon::WindowMaximize], image_size))).clicked() {
            appwindow_writer.write(AppWindowCommand::ToggleMaximize);
        }

        // Minimize
        if ui.add(egui::Button::image((icon_textures[&Icon::WindowMinimize], image_size))).clicked() {
            appwindow_writer.write(AppWindowCommand::Minimize);
        }
    });
}
