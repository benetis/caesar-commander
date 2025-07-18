use std::path::PathBuf;
use log::info;
use tokio::sync::mpsc;
use crate::file_system::file_mutator::FileMutator;
use crate::file_system::navigator::Navigator;
use crate::file_system::watcher::FileWatcher;
use crate::model::{Column};
use crate::model::pane_controls::PaneControlsEvent;
use crate::ui::file_pane::view::{View, NavigatedEvent};

pub struct Container {
    pub view: View,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
    watcher: FileWatcher,
}

impl Container {
    pub fn new(navigator: Navigator) -> Self {
        let items = navigator.list_contents();
        let columns = Self::default_columns();
        let (tx, rx) = mpsc::channel(1);
        let breadcrumbs = navigator.breadcrumbs();

        let current_path = navigator.current_path.clone();
        let watcher = FileWatcher::new(&tx, &*current_path)
            .expect("failed to init file watcher");

        let view = View::new(items, columns, tx, breadcrumbs);

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
                self.view.handle_selection_moved(*index, *selection, *additive, *direction);
            },
            NavigatedEvent::FilesUpdated => {
                self.refresh_items();
                if self.view.item_count() > 0 {
                    self.view.select_single(0);
                }
            },
        }
    }

    pub fn handle_pane_controls_event(&mut self, event: &PaneControlsEvent, destination: &PathBuf) {
        match event {
            PaneControlsEvent::MoveSelected => self.move_selected(destination),
        }
    }

    fn move_selected(&mut self, destination: &PathBuf) {
        let selected_items = self.view.get_selected_items();
        let original_max_index = self.view.get_selected_indices().iter().max().cloned().unwrap_or(0);

        for item in selected_items {
            let src = self.navigator.current_path.join(&item.name);
            let dst = destination.join(&item.name);

            info!("Moving {:?} -> {:?}", src, dst);

            if let Err(e) = FileMutator::durable_move(&src, &dst) {
                info!("Failed to move {:?}: {:?}", src, e);
            }
        }

        self.refresh_items();

        let count = self.view.item_count();
        if count > 0 {
            let next_index = (original_max_index + 1).min(count - 1);
            self.view.select_single(next_index);
        }
    }

    pub(crate) fn refresh_items(&mut self) {
        let items = self.navigator.list_contents();
        let breadcrumbs = self.navigator.breadcrumbs();
        self.view.update_contents(items, breadcrumbs);
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