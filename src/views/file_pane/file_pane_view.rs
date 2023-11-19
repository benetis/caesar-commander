use tokio::sync::mpsc;
use crate::model::*;
use crate::views::file_pane::file_pane::NavigatedEvent;
use egui::*;
use egui::text::LayoutJob;
use crate::model::*;

pub struct FilePaneView {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
    pub sender: mpsc::Sender<NavigatedEvent>,
    pub breadcrumbs: Vec<String>,
}


impl FilePaneView {
    pub fn ui(&mut self, ui: &mut Ui, id_source: &str, focused: bool) {

        if focused {
            self.handle_arrow_down(ui);
            self.handle_arrow_up(ui);
            self.handle_enter(ui);
            self.handle_backspace(ui);
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("📂");
                for crumb in &self.breadcrumbs {
                    ui.label(format!("{} /", crumb));
                }
            });

            ui.separator();

            self.draw_headers(ui);

            for item in &self.items {
                self.draw_item(ui, item);
            }
        });
    }

    fn draw_headers(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for col in &self.columns {
                ui.horizontal_wrapped(|ui| {
                    ui.set_width(col.width);
                    ui.label(&col.name);
                });
            }
        });
    }

    fn draw_item(&self, ui: &mut Ui, item: &Item) {
        let row_height = ui.text_style_height(&TextStyle::Body);
        let row_start = ui.cursor().min;
        let row_end = row_start + vec2(ui.max_rect().max.x, row_height);

        if item.selected {
            let rect = Rect::from_min_max(row_start, row_end);
            ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(230, 230, 230)); // Slightly darker background
        }

        ui.horizontal(|ui| {
            for col in &self.columns {
                ui.horizontal_wrapped(|ui| {
                    ui.set_width(col.width);
                    self.draw_item_cell(ui, item, &col.name);
                });
            }
        });
    }

    fn draw_item_cell(&self, ui: &mut Ui, item: &Item, col_name: &str) {
        let content = match col_name {
            "Icon" => match item.item_type {
                ItemType::File => "📄".to_string(),
                ItemType::Directory => "📁".to_string(),
            },
            "Name" => format!("{}", item.name),
            "Size" => format!("{} bytes", item.size),
            "Modified" => item.modified.to_rfc2822(),
            _ => "".to_string(),
        };

        let mut job = LayoutJob::single_section(
            content.to_owned(),
            egui::TextFormat {
                ..Default::default()
            },
        );
        job.wrap = egui::text::TextWrapping {
            max_rows: 1,
            break_anywhere: true,
            overflow_character: None,
            ..Default::default()
        };


        ui.label(job);
    }


    fn navigate(&mut self, direction: isize) {
        let len = self.items.len() as isize;
        if len == 0 { return; }

        let current_index = self.items.iter().position(|item| item.selected)
            .unwrap_or(0) as isize;
        let new_index = (current_index + direction).rem_euclid(len) as usize;

        let event = NavigatedEvent::SelectedItem(new_index);
        self.sender.try_send(event).unwrap();
    }


    fn handle_backspace(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(egui::Key::Backspace)) {
            let event = NavigatedEvent::GoUpDirectory;
            self.sender.try_send(event).unwrap();
        }
    }

    fn handle_enter(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let selected_item = self.items.iter()
                .find(|item| item.selected && item.item_type == ItemType::Directory);
            if let Some(item) = selected_item {
                let path = item.path.clone();
                let event = NavigatedEvent::OpenDirectory(path);
                self.sender.try_send(event).unwrap();
            }
        }
    }

    fn handle_arrow_up(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.navigate(-1);
        }
    }

    fn handle_arrow_down(&mut self, ui: &mut Ui) {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.navigate(1);
        }
    }
}