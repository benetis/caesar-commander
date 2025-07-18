use std::collections::BTreeSet;
use crate::model::*;
use crate::views::file_pane::file_pane::NavigatedEvent;
use egui::text::LayoutJob;
use egui::*;
use log::info;
use tokio::sync::mpsc;

pub struct FilePaneView {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
    pub sender: mpsc::Sender<NavigatedEvent>,
    pub breadcrumbs: Vec<String>,

    // Multiselect
    pub selected_indices: BTreeSet<usize>,
    pub cursor_index: usize,
    pub selection_anchor: Option<usize>,
}

impl FilePaneView {
    pub fn ui(&mut self, ui: &mut Ui, focused: bool) {
        self.handle_keyboard(ui, focused);

        let pane_rect = vec2(ui.available_width(), ui.available_height());

        ui.allocate_ui(pane_rect, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("📂");
                    for crumb in &self.breadcrumbs {
                        ui.label(format!("{} /", crumb));
                    }
                });
                ui.separator();
                self.draw_headers(ui);
                ui.separator();

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .id_salt(self as *const _ as usize)
                    .show(ui, |ui| {
                        for (i, item) in self.items.iter().enumerate() {
                            let selected = self.selected_indices.contains(&i);

                            self.draw_item(ui, item, focused, selected);
                        }
                    });
            });
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

    fn draw_item(&self, ui: &mut Ui, item: &Item, pane_focused: bool, selected: bool) {
        let row_start = ui.cursor().min;
        let row_end = row_start + vec2(ui.max_rect().max.x, Self::row_height(ui));
        let row_rect = Rect::from_min_max(row_start, row_end);

        if pane_focused && selected {
            ui.painter().rect_stroke(
                row_rect,
                0.0,
                Stroke::new(1.0, Color32::GRAY),
                StrokeKind::Inside,
            );

            ui.scroll_to_rect(row_rect, None);
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
        if len == 0 {
            return;
        }

        let current_index = self.cursor_index as isize;
        let new_index = (current_index + direction).rem_euclid(len) as usize;

        let event = NavigatedEvent::SelectionMoved {
            index: new_index,
            multi: false,
        };
        match self.sender.try_send(event) {
            Ok(_) => {}
            Err(e) => {
                info!("Could not send event: {:?}", e.to_string());
            }
        }
    }

    fn handle_keyboard(&mut self, ui: &mut Ui, focused: bool) {
        let mut _event_handled = false;

        if focused {
            _event_handled |= self.handle_arrow_down(ui);
            _event_handled |= self.handle_arrow_up(ui);
            _event_handled |= self.handle_enter(ui);
            _event_handled |= self.handle_backspace(ui);
            _event_handled |= self.handle_page_down(ui);
            _event_handled |= self.handle_page_up(ui);
        }
    }

    fn handle_backspace(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(egui::Key::Backspace)) {
            let event = NavigatedEvent::TraversedUp;
            self.sender.try_send(event).unwrap();
            true
        } else {
            false
        }
    }

    fn handle_enter(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {

            let selected_item = self.items.get(self.cursor_index)
                .filter(|item| item.item_type == ItemType::Directory);

            if let Some(item) = selected_item {
                let path = item.path.clone();
                let event = NavigatedEvent::DirectoryOpened(path);
                self.sender.try_send(event).unwrap();
            }
            true
        } else {
            false
        }
    }

    fn handle_arrow_down(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            let len = self.items.len();
            if len == 0 { return true; }
            let shift = ui.input(|i| i.modifiers.shift);

            let new_index = (self.cursor_index + 1).min(len - 1);
            let event = NavigatedEvent::SelectionMoved {
                index: new_index,
                multi: shift,
            };
            let _ = self.sender.try_send(event);
            true
        } else {
            false
        }
    }

    fn handle_arrow_up(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            let len = self.items.len();
            if len == 0 { return true; }
            let shift = ui.input(|i| i.modifiers.shift);
            let new_index = self.cursor_index.saturating_sub(1);
            let event = NavigatedEvent::SelectionMoved {
                index: new_index,
                multi: shift,
            };
            let _ = self.sender.try_send(event);
            true
        } else {
            false
        }
    }


    fn handle_page_down(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::PageDown)) {
            self.navigate(Self::page_step(ui));
            true
        } else {
            false
        }
    }

    fn handle_page_up(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i: &InputState| i.key_pressed(Key::PageUp)) {
            self.navigate(-Self::page_step(ui));
            true
        } else {
            false
        }
    }

    pub fn select_single(&mut self, index: usize) {
        self.selected_indices.clear();
        self.selected_indices.insert(index);
        self.cursor_index = index;
        self.selection_anchor = Some(index);
    }

    pub fn select_range(&mut self, anchor: usize, index: usize) {
        let (start, end) = if anchor <= index { (anchor, index) } else { (index, anchor) };
        self.selected_indices.clear();
        for i in start..=end {
            self.selected_indices.insert(i);
        }
        self.cursor_index = index;
        self.selection_anchor = Some(anchor);
    }

    pub fn toggle_selection(&mut self, index: usize) {
        if !self.selected_indices.insert(index) {
            self.selected_indices.remove(&index);
        }
        self.cursor_index = index;
    }

    fn selected_items(&self) -> impl Iterator<Item=&Item> {
        self.selected_indices.iter().filter_map(move |&i| self.items.get(i))
    }

    fn row_height(ui: &Ui) -> f32 {
        ui.text_style_height(&TextStyle::Body) + 6.0
    }

    fn page_step(ui: &Ui) -> isize {
        let rh = Self::row_height(ui);
        let count = (ui.available_height() / rh).floor() as isize;
        if count >= 1 { count } else { 1 }
    }
}
