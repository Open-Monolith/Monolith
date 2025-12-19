use eframe::{NativeOptions, egui};

mod app;
mod tabs;
mod utils;
mod widgets;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Monolith",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
