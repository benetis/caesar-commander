use egui::*;
use crate::views::double_pane::double_pane::FocusState;
use crate::views::file_pane::file_pane::FilePane;

pub struct DoublePaneView {
    pub left_file_pane: FilePane,
    pub right_file_pane: FilePane,
    pub focus_state: FocusState,
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
            ui.with_layout(Layout::left_to_right(Align::Center).with_main_justify(false), |ui| {
                ui.allocate_ui(Vec2::new(ui.available_width() / 2.0, ui.available_height()), |ui| {
                    self.left_file_pane.view.ui(ui, self.focus_state.is_left());
                });
            });

            ui.with_layout(Layout::left_to_right(Align::Center).with_main_justify(false), |ui| {
                ui.allocate_ui(Vec2::new(ui.available_width() / 2.0, ui.available_height()), |ui| {
                    self.right_file_pane.view.ui(ui, self.focus_state.is_right());
                });
            });
        });

        self.handle_focus_switch(ui);
    }

    fn handle_focus_switch(&mut self, ui: &mut Ui) {
        self.handle_left_arrow(ui);
        self.handle_right_arrow(ui);
        self.handle_tab(ui);
    }

    fn handle_left_arrow(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(Key::ArrowLeft)) {
            self.focus_state = FocusState::LeftPane;
        }
    }

    fn handle_right_arrow(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(Key::ArrowRight)) {
            self.focus_state = FocusState::RightPane;
        }
    }

    fn handle_tab(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(Key::Tab)) {
            self.focus_state.next();
        }
    }
}