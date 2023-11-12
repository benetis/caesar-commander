use chrono::{DateTime, Local};
use crate::model::*;

pub struct FilePane {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
}

impl Default for FilePane {
    fn default() -> Self {
        Self {
            items: vec![
                Item {
                    name: "file1.txt".to_string(),
                    path: Default::default(),
                    selected: false,
                    item_type: ItemType::File,
                    size: 1024,
                    modified: DateTime::default(),
                },
                Item {
                    name: "file directory".to_string(),
                    path: Default::default(),
                    selected: false,
                    item_type: ItemType::Directory,
                    size: 1337,
                    modified: DateTime::default()
                },
            ],
            columns: vec![
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
}
