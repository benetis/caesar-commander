
use std::collections::BTreeSet;
use std::path::PathBuf;

use tokio::sync::mpsc;
use log::info;
use egui::text::LayoutJob;
use egui::*;

use crate::model::*;
use crate::model::pane_controls::PaneControlsEvent;
use crate::file_system::file_mutator::FileMutator;
use crate::file_system::navigator::Navigator;
use crate::file_system::watcher::FileWatcher;

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

    fn handle_keyboard(&mut self, ui: &mut Ui, focused: bool) {
        if !focused { return; }

        let _ = self.handle_backspace(ui)
            | self.handle_enter(ui)
            | self.handle_arrow_up(ui)
            | self.handle_arrow_down(ui)
            | self.handle_page_up(ui)
            | self.handle_page_down(ui);
    }

    fn handle_backspace(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::Backspace)) {
            let _ = self.sender.try_send(NavigatedEvent::TraversedUp);
            true
        } else { false }
    }

    fn handle_enter(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::Enter)) {
            if let Some(item) = self.items.get(self.cursor_index)
                .filter(|it| it.item_type == ItemType::Directory) {
                let _ = self.sender.try_send(NavigatedEvent::DirectoryOpened(item.path.clone()));
            }
            true
        } else { false }
    }

    fn handle_arrow_down(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
            self.move_cursor(ui, 1, MoveDirection::Down);
            true
        } else { false }
    }

    fn handle_arrow_up(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
            self.move_cursor(ui, -1, MoveDirection::Up);
            true
        } else { false }
    }

    fn handle_page_down(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::PageDown)) {
            self.navigate(Self::page_step(ui));
            true
        } else { false }
    }

    fn handle_page_up(&mut self, ui: &mut Ui) -> bool {
        if ui.input(|i| i.key_pressed(Key::PageUp)) {
            self.navigate(-Self::page_step(ui));
            true
        } else { false }
    }

    fn move_cursor(&mut self, ui: &Ui, delta: isize, direction: MoveDirection) {
        let len = self.items.len();
        if len == 0 { return; }

        let new_index = ((self.cursor_index as isize) + delta)
            .clamp(0, (len - 1) as isize) as usize;

        let shift = ui.input(|i| i.modifiers.shift);
        let ctrl = ui.input(|i| i.modifiers.ctrl);

        let _ = self.sender.try_send(NavigatedEvent::SelectionMoved {
            index: new_index,
            selection: shift,
            additive: ctrl,
            direction: Some(direction),
        });
    }

    fn navigate(&mut self, amount: isize) {
        let len = self.items.len() as isize;
        if len == 0 { return; }

        let current_index = self.cursor_index as isize;
        let new_index = (current_index + amount).rem_euclid(len) as usize;

        let _ = self.sender.try_send(NavigatedEvent::SelectionMoved {
            index: new_index,
            selection: false,
            additive: false,
            direction: None,
        });
    }

    fn row_height(ui: &Ui) -> f32 {
        ui.text_style_height(&TextStyle::Body) + 6.0
    }

    fn page_step(ui: &Ui) -> isize {
        let rh = Self::row_height(ui);
        let count = (ui.available_height() / rh).floor() as isize;
        count.max(1)
    }
}

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
    watcher: FileWatcher,
}

impl FilePane {
    pub fn new(navigator: Navigator) -> Self {
        let items = navigator.list_contents();
        let columns = Self::default_columns();
        let (tx, rx) = mpsc::channel(1);
        let breadcrumbs = navigator.breadcrumbs();

        let current_path = navigator.current_path.clone();
        let watcher = FileWatcher::new(&tx, &current_path)
            .expect("failed to init file watcher");

        let mut view = FilePaneView {
            items,
            columns,
            sender: tx.clone(),
            breadcrumbs,

            // Multiselect
            selected_indices: BTreeSet::new(),
            cursor_index: 0,
            selection_anchor: Some(0),
            last_direction: None,
        };

        view.move_cursor_to_first();

        Self { view, navigator, receiver: rx, watcher }
    }

    pub fn handle_navigation_event(&mut self, event: &NavigatedEvent) {
        match event {
            NavigatedEvent::DirectoryOpened(path) => {
                self.navigator.open_dir(path);
                let _ = self.watcher.watch_path(&self.navigator.current_path);
                self.refresh_items();
                self.view.select_single(0);
            },
            NavigatedEvent::TraversedUp => {
                self.navigator.go_up();
                let _ = self.watcher.watch_path(&self.navigator.current_path);
                self.refresh_items();
                self.view.select_single(0);
            },
            NavigatedEvent::SelectionMoved { index, selection, additive, direction } => {
                let prev_dir = self.view.last_direction;
                self.view.last_direction = *direction;

                if *selection && *additive {
                    self.reset_anchor_if_diff_dir(direction, prev_dir);
                    let anchor = self.view.selection_anchor.unwrap_or(self.view.cursor_index);
                    self.view.add_range_to_selection(anchor, *index);
                } else if *selection {
                    let anchor = self.view.selection_anchor.unwrap_or(self.view.cursor_index);
                    self.view.select_range(anchor, *index);
                } else {
                    self.view.cursor_index = *index;
                    self.view.selection_anchor = Some(*index);
                }
            },
            NavigatedEvent::FilesUpdated => {
                self.refresh_items();
                if !self.view.items.is_empty() {
                    self.view.select_single(0);
                }
            },
        }
    }

    fn reset_anchor_if_diff_dir(&mut self, direction: &Option<MoveDirection>, prev_dir: Option<MoveDirection>) {
        if prev_dir != *direction {
            self.view.selection_anchor = Some(self.view.cursor_index);
        }
    }

    pub fn handle_pane_controls_event(&mut self, event: &PaneControlsEvent, destination: &PathBuf) {
        match event {
            PaneControlsEvent::MoveSelected => self.move_selected(destination),
        }
    }

    fn move_selected(&mut self, destination: &PathBuf) {
        let selected: Vec<usize> = self.view.selected_indices.iter().cloned().collect();

        for &i in &selected {
            if let Some(item) = self.view.items.get(i) {
                let src = self.navigator.current_path.join(&item.name);
                let dst = destination.join(&item.name);

                info!("Moving {:?} -> {:?}", src, dst);

                if let Err(e) = FileMutator::durable_move(&src, &dst) {
                    info!("Failed to move {:?}: {:?}", src, e);
                }
            }
        }

        self.refresh_items();

        let count = self.view.items.len();
        if count > 0 {
            let next = selected.iter().max().map(|x| x + 1).unwrap_or(0);
            let index = next.min(count - 1);
            self.view.select_single(index);
        }
    }

    pub(crate) fn refresh_items(&mut self) {
        self.view.items = self.navigator.list_contents();
        self.view.breadcrumbs = self.navigator.breadcrumbs();
    }


    fn default_columns() -> Vec<Column> {
        vec![
            Column { name: "Icon".into(),     width: 30.0 },
            Column { name: "Name".into(),     width: 100.0 },
            Column { name: "Size".into(),     width: 60.0 },
            Column { name: "Modified".into(), width: 200.0 },
        ]
    }
}
