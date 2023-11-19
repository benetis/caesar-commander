use tokio::sync::mpsc;
use crate::model::pane_controls::PaneControlsEvent;
use egui::*;

pub struct PaneControlsView {
    pub sender: mpsc::Sender<PaneControlsEvent>,
}

impl PaneControlsView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Controls");
        let _ = ui.button("Move F6");
        
        if ui.input(|i| i.key_pressed(Key::F6)) {
            self.sender.try_send(PaneControlsEvent::MoveSelected).unwrap();
        }
    }
}