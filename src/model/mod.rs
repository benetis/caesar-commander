pub mod focus_state;

use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(PartialEq)]
pub enum ItemType {
    File,
    Directory,
}

pub struct Item {
    pub name: String,
    pub path: PathBuf,
    pub selected: bool,
    pub item_type: ItemType,
    pub size: u64,
    pub modified: DateTime<Local>,
}

pub struct Column {
    pub name: String,
    pub width: f32,
}
