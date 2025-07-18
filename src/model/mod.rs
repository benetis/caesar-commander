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
    pub item_type: ItemType,
    pub size: u64,
    pub modified: DateTime<Local>,
}

pub struct Column {
    pub name: String,
    pub width: f32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveDirection {
    Up,
    Down,
}