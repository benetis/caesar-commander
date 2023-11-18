use chrono::{DateTime, Local};
use tokio::sync::mpsc;
use crate::model::*;
use crate::views::file_pane::file_pane::NavigatedEvent;
use egui::*;
use crate::file_system::navigator::Navigator;
use crate::model::*;

pub struct FilePaneView {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
    pub sender: mpsc::Sender<NavigatedEvent>,
}


impl FilePaneView {
    pub fn ui(&mut self, ui: &mut Ui) {
        Grid::new("file-views-1")
            .num_columns(self.columns.len())
            .striped(true)
            .show(ui, |ui| {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    self.navigate(1);
                }

                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    self.navigate(-1);
                }

                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let selected_item = self.items.iter()
                        .find(|item| item.selected && item.item_type == ItemType::Directory);
                    if let Some(item) = selected_item {
                        let path = item.path.clone();
                        let event = NavigatedEvent::OpenDirectory(path);
                        self.sender.try_send(event).unwrap();
                    }
                }

                if ui.input(|i| i.key_pressed(egui::Key::Backspace)) {
                    let event = NavigatedEvent::GoUp;
                    self.sender.try_send(event).unwrap();
                }

                // Headers
                for col in &self.columns {
                    ui.label(&col.name);
                }
                ui.end_row();


                for item in &self.items {
                    Self::draw_item_widget(ui, item);
                }
            });
    }

    fn draw_item_widget(ui: &mut Ui, item: &Item) {
        match item.item_type {
            ItemType::File => ui.label("📄"),
            ItemType::Directory => ui.label("📁"),
        };

        let prefix = if item.selected { "🔹" } else { "    " };

        ui.label(format!("{prefix}{}", &item.name));
        ui.label(format!("{} bytes", item.size));
        ui.label(&item.modified.to_rfc2822());
        ui.end_row();
    }


    fn navigate(&mut self, direction: isize) {
        let len = self.items.len() as isize;
        let current_index = self.items.iter().position(|item| item.selected)
            .unwrap_or(0) as isize;
        let new_index = (current_index + direction).rem_euclid(len) as usize;

        for item in &mut self.items {
            item.selected = false;
        }
        self.items[new_index].selected = true;
    }
}