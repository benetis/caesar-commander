
use egui::*;
use crate::views::double_pane::double_pane::FocusState;
use crate::views::file_pane::file_pane::FilePane;

pub struct DoublePaneView {
    pub left_file_pane: FilePane,
    pub right_file_pane: FilePane,
    pub focus_state: FocusState
}


impl DoublePaneView {
    pub fn ui(&mut self, ui: &mut Ui) {

        while let Ok(event) = self.left_file_pane.receiver.try_recv() {
            self.left_file_pane.handle_navigation_event(&event);
        }

        while let Ok(event) = self.right_file_pane.receiver.try_recv() {
            self.right_file_pane.handle_navigation_event(&event);
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.left_file_pane.view.ui(ui, "left", self.focus_state.is_left());
            });
            ui.vertical(|ui| {
                self.right_file_pane.view.ui(ui, "right", self.focus_state.is_right());
            });
        });


    }
}