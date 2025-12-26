use crate::app::MyApp;
use eframe::egui;

pub(crate) fn menu_bar(app: &mut MyApp, ctx: &egui::Context, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
    egui::MenuBar::new().ui(ui, |ui| {
        file_menu(ctx, ui);
        edit_menu(ctx, ui);
        window_menu(ctx, ui);
        view_menu(ctx, ui);

        ui.add_space(25.);

        workspace_buttons(ui);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            window_controls(ctx, ui);
            search_bar(app, ui);
            topbar_resize(ctx, ui);
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

fn search_bar(app: &mut MyApp, ui: &mut egui::Ui) {
    egui::Frame::NONE
        .fill(egui::Color32::from_hex("#252525").unwrap())
        .corner_radius(egui::CornerRadius::same(4))
        .inner_margin(egui::Margin::symmetric(6, 3))
        .stroke(egui::Stroke::NONE)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_text)
                        .frame(false)
                        .hint_text("Search...")
                        .desired_width(150.0)
                        .text_color(egui::Color32::LIGHT_GRAY),
                );
                ui.label(egui::RichText::new("🔍").color(egui::Color32::GRAY));
            });
        });
}

fn window_controls(ctx: &egui::Context, ui: &mut egui::Ui) {
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

    if ui.button("❌").clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    if ui.button(if is_maximized { "🗗" } else { "🗖" }).clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
    }

    if ui.button("🗕").clicked() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

fn topbar_resize(ctx: &egui::Context, ui: &mut egui::Ui) {
    let available_width = ui.available_width();
    
    if available_width > 0.0 {
        let (_rect, response) = ui.allocate_exact_size(
            egui::vec2(available_width, ui.available_height()),
            egui::Sense::click_and_drag(),
        );

        if response.double_clicked() {
            let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
        }

        if response.dragged() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
    }
}
