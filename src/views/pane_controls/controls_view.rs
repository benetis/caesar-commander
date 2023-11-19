use tokio::sync::mpsc;
use crate::model::pane_controls::PaneControlsEvent;
use egui::*;

pub struct PaneControlsView {
    pub sender: mpsc::Sender<PaneControlsEvent>,
}

impl PaneControlsView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Controls");
        if ui.button("Move F6").clicked() {
            self.sender.try_send(PaneControlsEvent::MoveSelected).unwrap();
        }
    }
}