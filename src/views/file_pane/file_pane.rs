use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::file_system::navigator::Navigator;
use crate::model::{Column, Item};
use crate::views::file_pane::file_pane_view::FilePaneView;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>,
}

pub enum NavigatedEvent {
    OpenDirectory(PathBuf),
    GoUpDirectory,
    SelectedItem(usize),
}


impl FilePane {
    pub fn new(navigator: Navigator) -> Self {
        let items = FilePane::select_item(&navigator.list_contents(), 0);
        let columns = Self::columns();
        let mpsc = mpsc::channel(1);
        let breadcrumbs = navigator.breadcrumbs();

        Self {
            view: FilePaneView {
                items,
                columns,
                sender: mpsc.0,
                breadcrumbs,
            },
            navigator,
            receiver: mpsc.1,
        }
    }

    pub fn handle_navigation_event(&mut self, event: &NavigatedEvent) {
        match event {
            NavigatedEvent::OpenDirectory(path) => {
                self.navigator.open_dir(&path);
                self.update_items();
                self.update_selected_item(0);
            }
            NavigatedEvent::GoUpDirectory => {
                self.navigator.go_up();
                self.update_items();
                self.update_selected_item(0);
            }
            NavigatedEvent::SelectedItem(index) => {
                self.update_selected_item(*index);
            }
        }
    }

    fn update_items(&mut self) {
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
                width: 40.0,
            },
            Column {
                name: "Name".to_string(),
                width: 200.0,
            },
            Column {
                name: "Size".to_string(),
                width: 100.0,
            },
            Column {
                name: "Modified".to_string(),
                width: 200.0,
            },
        ]
    }
}