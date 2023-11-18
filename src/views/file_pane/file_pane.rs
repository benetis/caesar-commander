use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::file_system::navigator::Navigator;
use crate::model::Column;
use crate::views::file_pane::file_pane_view::FilePaneView;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigatedEvent>
}

pub enum NavigatedEvent {
    OpenDirectory(PathBuf),
    GoUp,
}


impl FilePane {
    pub fn new(navigator: Navigator) -> Self {
        let items = navigator.list_contents();
        let columns = Self::columns();
        let mpsc = mpsc::channel(1);

        Self {
            view: FilePaneView {
                items,
                columns,
                sender: mpsc.0
            },
            navigator,
            receiver: mpsc.1
        }
    }

    pub fn handle_navigation_event(&mut self, event: &NavigatedEvent) {
        match event {
            NavigatedEvent::OpenDirectory(path) => {
                self.navigator.open_dir(&path);
                self.update_items();
            },
            NavigatedEvent::GoUp => {
                self.navigator.go_up();
                self.update_items();
            },
        }
    }

    fn update_items(&mut self) {
        self.view.items = self.navigator.list_contents();
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