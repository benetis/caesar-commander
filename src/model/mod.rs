pub mod pane_controls;
pub mod params;

use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(PartialEq, Clone)]
pub enum ItemType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub path: PathBuf,
    pub selected: bool,
    pub item_type: ItemType,
    pub size: u64,
    pub modified: DateTime<Local>,
}

impl Item {
    pub fn selected(&self) -> Self {
        Item {
            selected: true,
            ..(*self).clone()
        }
    }

    pub fn deselected(&self) -> Self {
        Item {
            selected: false,
            ..(*self).clone()
        }
    }
}

pub struct Column {
    pub name: String,
    pub width: f32,
}
