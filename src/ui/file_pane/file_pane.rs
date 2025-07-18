use std::collections::BTreeSet;
use std::path::PathBuf;

use egui::text::LayoutJob;
use egui::*;
use tokio::sync::mpsc;

use crate::model::*;
use crate::ui::keyboard::keyboard;

#[derive(Debug)]
pub enum NavigatedEvent {
    DirectoryOpened(PathBuf),
    TraversedUp,
    SelectionMoved {
        index: usize,
        selection: bool,
        additive: bool,
        direction: Option<MoveDirection>,
    },
    FilesUpdated,
}

pub struct FilePaneView {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
    pub sender: mpsc::Sender<NavigatedEvent>,
    pub breadcrumbs: Vec<String>,

    // Multiselect
    pub selected_indices: BTreeSet<usize>,
    pub cursor_index: usize,
    pub selection_anchor: Option<usize>,
    pub last_direction: Option<MoveDirection>,
}

impl FilePaneView {
    pub fn ui(&mut self, ui: &mut Ui, focused: bool) {
        keyboard::handle(self, ui, focused);

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
                            let is_cursor = self.cursor_index == i;

                            self.draw_item(ui, item, focused, selected, is_cursor);
                        }
                    });
            });
        });
    }

    pub fn move_cursor_to_first(&mut self) {
        if !self.items.is_empty() {
            self.cursor_index = 0;
            self.selection_anchor = Some(0);
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

    pub fn add_range_to_selection(&mut self, anchor: usize, index: usize) {
        let (start, end) = if anchor <= index { (anchor, index) } else { (index, anchor) };
        for i in start..=end {
            self.selected_indices.insert(i);
        }
        self.cursor_index = index;
        self.selection_anchor = Some(anchor);
    }

    pub fn page_step(ui: &Ui) -> isize {
        let rh = Self::row_height(ui);
        let count = (ui.available_height() / rh).floor() as isize;
        count.max(1)
    }

    fn draw_headers(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for col in &self.columns {
                ui.allocate_ui(vec2(col.width, ui.available_height()), |ui| {
                    ui.label(&col.name);
                });
            }
        });
    }

    fn draw_item(&self, ui: &mut Ui, item: &Item, pane_focused: bool, selected: bool, is_cursor: bool) {
        let row_start = ui.cursor().min;
        let row_end = row_start + vec2(ui.max_rect().max.x, Self::row_height(ui));
        let row_rect = Rect::from_min_max(row_start, row_end);

        // Cursor outline
        if pane_focused && is_cursor {
            ui.painter().rect_stroke(
                row_rect,
                0.0,
                Stroke::new(2.0, Color32::LIGHT_BLUE),
                StrokeKind::Inside,
            );
            ui.scroll_to_rect(row_rect, None);
        }

        ui.horizontal(|ui| {
            for col in &self.columns {
                ui.allocate_ui(vec2(col.width, Self::row_height(ui)), |ui| {
                    self.draw_item_cell(ui, item, &col.name, selected);
                });
            }
        });
    }

    fn draw_item_cell(&self, ui: &mut Ui, item: &Item, col_name: &str, selected_text: bool) {
        let content = match col_name {
            "Icon" => match item.item_type {
                ItemType::File => "📄".to_string(),
                ItemType::Directory => "📁".to_string(),
            },
            "Name" => item.name.clone(),
            "Size" => format!("{} bytes", item.size),
            "Modified" => item.modified.to_rfc2822(),
            _ => String::new(),
        };

        let mut text_color = ui.visuals().text_color();
        if selected_text {
            text_color = Color32::from_rgb(60, 120, 255);
        }

        let mut job = LayoutJob::single_section(
            content,
            TextFormat {
                color: text_color,
                ..Default::default()
            },
        );
        job.wrap = text::TextWrapping {
            max_rows: 1,
            break_anywhere: true,
            overflow_character: None,
            ..Default::default()
        };

        ui.label(job);
    }

    fn row_height(ui: &Ui) -> f32 {
        ui.text_style_height(&TextStyle::Body) + 6.0
    }
}
