use chrono::{DateTime, Local};
use crate::model::*;

pub struct FilePaneView {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
}

impl Default for FilePaneView {
    fn default() -> Self {
        Self {
            items: vec![
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
