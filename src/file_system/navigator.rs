use std::fs;
use std::path::{Path, PathBuf};
use crate::views::file_pane::model::Item;

pub struct Navigator {
    current_path: PathBuf,
}

impl Navigator {
    pub fn new(initial_path: PathBuf) -> Self {
        Navigator { current_path: initial_path }
    }

    pub fn open_folder(&mut self, path: &Path) {
        self.current_path = path.to_path_buf();
    }

    pub fn list_contents(&self) -> Vec<Item> {
        todo!()
    }
}