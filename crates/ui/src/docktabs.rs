use eframe::egui;
use crate::tabs;

#[derive(Clone, PartialEq, Debug)]
pub enum TabKind {
    SceneView,
    Explorer,
    Console,
    Properties,
}

pub const ALL_TAB_KINDS: [TabKind; 4] = [
    TabKind::SceneView,
    TabKind::Explorer,
    TabKind::Console,
    TabKind::Properties,
];

#[derive(Clone, PartialEq, Debug)]
pub struct MyTab {
    pub kind: TabKind,
    pub title: String,
    pub id: u32,
}

pub struct MyTabViewer {}

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = MyTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        egui::ComboBox::from_id_salt("tab_kind_selector")
            .selected_text(format!("{:?}", tab.kind))
            .show_ui(ui, |ui| {
                for kind in ALL_TAB_KINDS {
                    if ui
                        .selectable_value(&mut tab.kind, kind.clone(), format!("{:?}", kind))
                        .clicked()
                    {
                        tab.title = format!("{:?}", kind);
                    }
                }
            });

        match tab.kind {
            TabKind::Explorer => {
                tabs::explorer::show(ui, tab);
            }
            TabKind::Console => {
                tabs::console::show(ui, tab);
            }
            TabKind::Properties => {
                tabs::properties::show(ui, tab);
            }
            TabKind::SceneView => {
                tabs::sceneview::show(ui, tab);
            }
        }
    }
}


