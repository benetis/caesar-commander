use egui::*;
use crate::ui::double_pane::double_pane::FocusState;
use crate::ui::file_pane;
use crate::ui::pane_controls::controls::PaneControls;

pub struct DoublePaneView {
    pub left_file_pane: file_pane::Container,
    pub right_file_pane: file_pane::Container,
    pub focus_state: FocusState,
    pub pane_controls: PaneControls,
}


impl DoublePaneView {
    pub fn ui(&mut self, ui: &mut Ui) {
        self.handle_navigation_event();
        self.handle_pane_controls_event();

        const CONTROLS_HEIGHT: f32 = 80.0;
        let pane_height = ui.available_height() - CONTROLS_HEIGHT;

        ui.horizontal(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center).with_main_justify(false), |ui| {
                ui.allocate_ui(Vec2::new(ui.available_width() / 2.0, pane_height), |ui| {
                    self.left_file_pane.view.ui(ui, self.focus_state.is_left());
                });
            });

            ui.with_layout(Layout::left_to_right(Align::Center).with_main_justify(false), |ui| {
                ui.allocate_ui(Vec2::new(ui.available_width() / 2.0, pane_height), |ui| {
                    self.right_file_pane.view.ui(ui, self.focus_state.is_right());
                });
            });
        });

        ui.separator();

        self.pane_controls.view.ui(ui);

        self.handle_focus_switch(ui);
    }

    fn handle_navigation_event(&mut self) {
        while let Ok(event) = self.left_file_pane.receiver.try_recv() {
            self.left_file_pane.handle_navigation_event(&event);
        }

        while let Ok(event) = self.right_file_pane.receiver.try_recv() {
            self.right_file_pane.handle_navigation_event(&event);
        }
    }

    fn handle_pane_controls_event(&mut self) {
        while let Ok(event) = self.pane_controls.receiver.try_recv() {
            match self.focus_state {
                FocusState::LeftPane => {
                    self.left_file_pane.handle_pane_controls_event(
                        &event, &self.right_file_pane.navigator.current_path
                    );
                    self.right_file_pane.refresh_items();
                }
                FocusState::RightPane => {
                    self.right_file_pane.handle_pane_controls_event(
                        &event, &self.left_file_pane.navigator.current_path
                    );
                    self.left_file_pane.refresh_items();
                }
            }
        }
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