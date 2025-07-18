use crate::file_system::file_mutator::FileMutator;
use crate::file_system::navigator::Navigator;
use crate::file_system::watcher::FileWatcher;
use crate::model::pane_controls::PaneControlsEvent;
use crate::model::Column;
use crate::views::file_pane::file_pane_view::FilePaneView;
use std::collections::BTreeSet;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
    watcher: FileWatcher,
}

pub enum NavigatedEvent {
    DirectoryOpened(PathBuf),
    TraversedUp,
    SelectionMoved {
        index: usize,
        selection: bool,
    },
    FilesUpdated,
}


impl FilePane {
    pub fn new(navigator: Navigator) -> Self {
        let items = navigator.list_contents();
        let columns = Self::columns();
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
        };

        view.move_cursor_to_first();

        Self {
            view,
            navigator,
            receiver: rx,
            watcher,
        }
    }

    pub fn handle_navigation_event(&mut self, event: &NavigatedEvent) {
        match event {
            NavigatedEvent::DirectoryOpened(path) => {
                self.navigator.open_dir(&path);
                self.watcher.watch_path(&self.navigator.current_path).ok();
                self.refresh_items();
                self.view.select_single(0);
            }
            NavigatedEvent::TraversedUp => {
                self.navigator.go_up();
                self.watcher.watch_path(&self.navigator.current_path).ok();
                self.refresh_items();
                self.view.select_single(0);
            }
            NavigatedEvent::SelectionMoved { index, selection } => {
                if *selection {
                    // Shift is held: do a range selection from anchor to index
                    let anchor = self.view.selection_anchor.unwrap_or(self.view.cursor_index);
                    self.view.select_range(anchor, *index);
                } else {
                    self.view.cursor_index = *index;
                    self.view.selection_anchor = Some(*index);
                }
            }
            NavigatedEvent::FilesUpdated => {
                self.refresh_items();
                if !self.view.items.is_empty() {
                    self.view.select_single(0);
                }
            }
        }
    }

    pub fn handle_pane_controls_event(&mut self, event: &PaneControlsEvent, destination: &PathBuf) {
        match event {
            PaneControlsEvent::MoveSelected => {
                let selected_indices: Vec<usize> = self.view.selected_indices.iter().cloned().collect();

                for &i in &selected_indices {
                    if let Some(item) = self.view.items.get(i) {
                        let src = self.navigator.current_path.join(&item.name);
                        let dst = destination.join(&item.name);

                        println!("Moving {:?} to {:?}", src, dst);

                        if let Err(e) = FileMutator::durable_move(&src, &dst) {
                            println!("Failed to move {:?}: {:?}", src, e);
                        }
                    }
                }

                self.refresh_items();

                let count = self.view.items.len();
                if count > 0 {
                    let next = selected_indices.iter().max().map(|x| x+1).unwrap_or(0);
                    let index = next.min(count - 1);
                    self.view.select_single(index);
                }
            }
        }
    }

    pub fn refresh_items(&mut self) {
        self.view.items = self.navigator.list_contents();
        self.view.breadcrumbs = self.navigator.breadcrumbs();
    }

    fn columns() -> Vec<Column> {
        vec![
            Column {
                name: "Icon".to_string(),
                width: 30.0,
            },
            Column {
                name: "Name".to_string(),
                width: 100.0,
            },
            Column {
                name: "Size".to_string(),
                width: 60.0,
            },
            Column {
                name: "Modified".to_string(),
                width: 200.0,
            },
        ]
    }
}