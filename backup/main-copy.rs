use std::collections::HashMap;
 
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};
 
// -----------------------------
// 1) Trait for pluggable panes
// -----------------------------
trait PaneWidget {
    fn title(&self) -> egui::WidgetText;
    fn ui(&mut self, ui: &mut egui::Ui) -> UiResponse;
}
 
// -----------------------------
// 2) Pane key + pane handle stored in egui_tiles
// -----------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PaneKey(u64);
 
#[derive(Clone, Debug)]
struct Pane {
    key: PaneKey,
}
 
// -----------------------------
// 3) Registry holding real pane widgets + state
// -----------------------------
#[derive(Resource)]
struct PaneRegistry {
    panes: HashMap<PaneKey, Box<dyn PaneWidget + Send + Sync>>,
}
 
impl PaneRegistry {
    fn new() -> Self {
        Self {
            panes: HashMap::new(),
        }
    }
 
    fn insert(&mut self, key: PaneKey, pane: impl PaneWidget + Send + Sync + 'static) {
        self.panes.insert(key, Box::new(pane));
    }
 
    fn get(&self, key: PaneKey) -> &dyn PaneWidget {
        self.panes.get(&key).expect("missing pane").as_ref()
    }
 
    fn get_mut(&mut self, key: PaneKey) -> &mut dyn PaneWidget {
        self.panes.get_mut(&key).expect("missing pane").as_mut()
    }
}
 
// -----------------------------
// 4) Tree resource
// -----------------------------
#[derive(Resource)]
struct DockTree {
    tree: Tree<Pane>,
}
 
// -----------------------------
// 5) Behavior delegates to registry
// -----------------------------
struct TreeBehavior<'a> {
    reg: &'a mut PaneRegistry,
}
 
impl<'a> Behavior<Pane> for TreeBehavior<'a> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        self.reg.get(pane.key).title()
    }
 
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: TileId,
        pane: &mut Pane,
    ) -> UiResponse {
        self.reg.get_mut(pane.key).ui(ui)
    }
}
 
// -----------------------------
// 6) Example panes
// -----------------------------
struct ConsolePane {
    filter: String,
}
 
impl Default for ConsolePane {
    fn default() -> Self {
        Self {
            filter: "hello".to_owned(),
        }
    }
}
 
impl PaneWidget for ConsolePane {
    fn title(&self) -> egui::WidgetText {
        "Console".into()
    }
 
    fn ui(&mut self, ui: &mut egui::Ui) -> UiResponse {
        ui.label("Console pane");
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.filter);
        });
        UiResponse::None
    }
}
 
struct PropertiesPane {
    enabled: bool,
}
 
impl Default for PropertiesPane {
    fn default() -> Self {
        Self { enabled: true }
    }
}
 
impl PaneWidget for PropertiesPane {
    fn title(&self) -> egui::WidgetText {
        "Properties".into()
    }
 
    fn ui(&mut self, ui: &mut egui::Ui) -> UiResponse {
        ui.label("Properties pane");
        ui.checkbox(&mut self.enabled, "Enabled");
        UiResponse::None
    }
}
 
struct ViewportPane {
    clicks: u32,
}
 
impl Default for ViewportPane {
    fn default() -> Self {
        Self { clicks: 0 }
    }
}
 
impl PaneWidget for ViewportPane {
    fn title(&self) -> egui::WidgetText {
        "Viewport".into()
    }
 
    fn ui(&mut self, ui: &mut egui::Ui) -> UiResponse {
        ui.label("Viewport pane");
        if ui.button("Click").clicked() {
            self.clicks += 1;
        }
        ui.label(format!("Clicks: {}", self.clicks));
        UiResponse::None
    }
}
 
// -----------------------------
// 7) Optional message example
// -----------------------------
#[derive(Message, Debug, Clone, Copy)]
enum UiCommand {
    Ping,
}
 
// -----------------------------
// 8) Startup: build registry + tree
// -----------------------------
fn setup(mut commands: Commands) {
    let mut reg = PaneRegistry::new();
    reg.insert(PaneKey(1), ConsolePane::default());
    reg.insert(PaneKey(2), PropertiesPane::default());
    reg.insert(PaneKey(3), ViewportPane::default());
 
    let mut tiles = Tiles::<Pane>::default();
    let p1 = tiles.insert_pane(Pane { key: PaneKey(1) });
    let p2 = tiles.insert_pane(Pane { key: PaneKey(2) });
    let p3 = tiles.insert_pane(Pane { key: PaneKey(3) });
 
    let root = tiles.insert_tab_tile(vec![p1, p2, p3]);
    let tree = Tree::new("my_tree", root, tiles);
 
    commands.insert_resource(reg);
    commands.insert_resource(DockTree { tree });
 
    // Primary egui context needs a camera.
    commands.spawn(Camera2d);
}
 
// -----------------------------
// 9) UI system: draw egui_tiles inside Bevy
// -----------------------------
fn dock_ui_system(
    mut contexts: EguiContexts,
    mut dock: ResMut<DockTree>,
    mut reg: ResMut<PaneRegistry>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };
 
    egui::CentralPanel::default().show(ctx, |ui| {
        let mut behavior = TreeBehavior { reg: &mut reg };
        dock.tree.ui(&mut behavior, ui);
    });
}
 
// -----------------------------
// 10) Optional message handler
// -----------------------------
fn handle_ui_commands(mut reader: MessageReader<UiCommand>) {
    for msg in reader.read() {
        match *msg {
            UiCommand::Ping => {
                info!("Ping");
            }
        }
    }
}
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_message::<UiCommand>()
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, dock_ui_system)
        .add_systems(Update, handle_ui_commands)
        .run();
}