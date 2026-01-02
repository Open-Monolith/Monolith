use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;

use mn_core::Tab;

pub struct MyTabViewer<'a> {
    // This is a small buffer owned by the UI crate for this frame.
    pub viewport_out: &'a mut Option<Rect>,
}

impl TabViewer for MyTabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport => "Viewport".into(),
            Tab::Inspector => "Inspector".into(),
            Tab::Hierarchy => "Hierarchy".into(),
            Tab::Assets => "Assets".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport => {
                let rect = ui.available_rect_before_wrap();
                if rect.width() < 0.0 && rect.height() < 0.0 { return } 
                // Reserve space so the layout doesn't collapse
                ui.allocate_rect(rect, egui::Sense::hover());
                
                // Set the rect for this frame
                *self.viewport_out = Some(rect);
                
                // Draw a placeholder background (optional, but good for debugging)
                // We use transparent so Bevy shows through, or dark grey to hide glitches
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::from_black_alpha(0), 
                );
            }
            Tab::Inspector => { ui.heading("Inspector"); }
            Tab::Hierarchy => { ui.heading("Hierarchy"); }
            Tab::Assets => { ui.heading("Assets"); }
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, Tab::Viewport)
    }
}