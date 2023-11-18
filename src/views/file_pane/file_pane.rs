use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::file_system::navigator::Navigator;
use crate::model::Column;
use crate::views::file_pane::file_pane_view::FilePaneView;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
    pub receiver: mpsc::Receiver<NavigateEvent>,
}

pub struct NavigateEvent {
    pub path: PathBuf,
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

                sender: mpsc.0,
            },
            navigator,
            receiver: mpsc.1,
        }
    }

    pub fn handle_navigation_events(&mut self, path_buf: &PathBuf) {
        self.navigator.open_folder(path_buf);
        self.update_items();
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