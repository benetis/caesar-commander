use std::collections::BTreeSet;
use std::path::PathBuf;
use log::info;
use tokio::sync::mpsc;
use crate::file_system::file_mutator::FileMutator;
use crate::file_system::navigator::Navigator;
use crate::file_system::watcher::FileWatcher;
use crate::model::{Column, MoveDirection};
use crate::model::pane_controls::PaneControlsEvent;
use crate::ui::file_pane::file_pane::{FilePaneView, NavigatedEvent};

pub struct FilePaneContainer {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
    watcher: FileWatcher,
}

impl FilePaneContainer {
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
