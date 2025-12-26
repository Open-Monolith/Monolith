use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

#[derive(Clone, PartialEq, Debug)]
pub enum TabKind {
    SceneView,
    Properties,
    Console,
    Explorer,
}

#[derive(Resource)]
pub struct UiLayout {
    pub state: DockState<TabKind>
}

impl Default for UiLayout {
    fn default() -> Self {
        let mut state = DockState::new(vec![TabKind::SceneView]);
        let surface  = state.main_surface_mut();
        // Split right for properties
        let [main, _right] = surface.split_right(
            NodeIndex::root(), 
            0.25, 
            vec![TabKind::Properties]
        );
        
        // Split bottom of main for console
        let [_main, _bottom] = surface.split_below(
            main,
            0.20, 
            vec![TabKind::Console]
        );
        
        Self { state }
    }
}

struct MonolithTabViewer<'a> {
    world_name: &'a str
}

impl<'a> egui_dock::TabViewer for MonolithTabViewer<'a> 
{
    type Tab = TabKind;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("{tab:?}").into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            TabKind::SceneView => {
                // This is where we will eventually draw the 3D scene texture
                ui.centered_and_justified(|ui| {
                    ui.label("3D Viewport (Render Target)");
                });
            }
            TabKind::Properties => {
                ui.heading("Properties");
                ui.label("Wall Height: 3000mm");
                if ui.button("Apply").clicked() {
                    println!("Applied changes to {}", self.world_name);
                }
            }
            TabKind::Console => {
                ui.label("> Lua console initialized.");
            }
            TabKind::Explorer => {
                ui.label("Project Tree");
            }
        }
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut layout: ResMut<UiLayout>
) {
    let ctx = contexts.ctx_mut().unwrap();

    let mut viewer = MonolithTabViewer {
        world_name: "Monolith",
    };

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::from_gray(20)))
        .show(ctx, |ui| {
            DockArea::new(&mut layout.state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut viewer);
        });
}