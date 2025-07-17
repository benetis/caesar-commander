use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::file_system::file_mutator::FileMutator;
use crate::file_system::navigator::Navigator;
use crate::file_system::watcher::FileWatcher;
use crate::model::{Column, Item};
use crate::model::pane_controls::PaneControlsEvent;
use crate::views::file_pane::file_pane_view::FilePaneView;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
    watcher: FileWatcher,
}

pub enum NavigatedEvent {
    DirectoryOpened(PathBuf),
    TraversedUp,
    SelectedItem(usize),
    FilesUpdated,
}


impl FilePane {
    pub fn new(navigator: Navigator) -> Self {
        let items = FilePane::select_item(&navigator.list_contents(), 0);
        let columns = Self::columns();
        let (tx, rx) = mpsc::channel(1);
        let breadcrumbs = navigator.breadcrumbs();

        let current_path = navigator.current_path.clone();
        let watcher = FileWatcher::new(&tx, &current_path)
            .expect("failed to init file watcher");

        Self {
            view: FilePaneView {
                items,
                columns,
                sender: tx.clone(),
                breadcrumbs,
            },
            navigator,
            receiver: rx,
            watcher,
        }
    }

    pub fn handle_navigation_event(&mut self, event: &NavigatedEvent) {
        match event {
            NavigatedEvent::DirectoryOpened(path) => {
                self.navigator.open_dir(&path);
                self.watcher
                    .watch_path(&self.navigator.current_path)
                    .ok();

                self.refresh_items();
                self.update_selected_item(0);
            }
            NavigatedEvent::TraversedUp => {
                self.navigator.go_up();
                self.watcher
                    .watch_path(&self.navigator.current_path)
                    .ok();

                self.refresh_items();
                self.update_selected_item(0);
            }
            NavigatedEvent::SelectedItem(index) => {
                self.update_selected_item(*index);
            }
            NavigatedEvent::FilesUpdated => {
                self.refresh_items();
                if !self.view.items.is_empty() {
                    self.update_selected_item(0);
                }
            }
        }
    }

    pub fn handle_pane_controls_event(&mut self, event: &PaneControlsEvent, destination: &PathBuf) {
        match event {
            PaneControlsEvent::MoveSelected => {
                let (index, selected_item) = self.view.items.iter().enumerate().find(|(_, item)| item.selected).unwrap();
                let current_file_full = self.navigator.current_path.join(&selected_item.name);

                println!("Moving {:?} to {:?}", current_file_full, destination.join(&selected_item.name));

                FileMutator::durable_move(
                    &current_file_full,
                    &destination.join(&selected_item.name)
                ).unwrap();

                self.refresh_items();

                if self.view.items.len() > 0 {
                    if index == self.view.items.len()   {
                        self.update_selected_item(index - 1);
                    } else {
                        self.update_selected_item(index);
                    }
                }
            }
        }
    }

    pub fn refresh_items(&mut self) {
        self.view.items = self.navigator.list_contents();
        self.view.breadcrumbs = self.navigator.breadcrumbs();
    }

    fn update_selected_item(&mut self, index: usize) {
        self.view.items = Self::select_item(&self.view.items, index);
    }

    fn select_item(items: &Vec<Item>, index: usize) -> Vec<Item> {
        items.iter().enumerate().map(|(i, item)| {
            if i == index {
                item.selected()
            } else {
                item.deselected()
            }
        }).collect()
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