use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::model::*;
use dirs;

#[derive(Clone)]
pub struct Navigator {
    current_path: PathBuf,
}

impl Navigator {
    pub fn new(initial_path: PathBuf) -> Self {
        Navigator { current_path: initial_path }
    }

    pub fn open_folder(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
    }

    pub fn list_contents(&self) -> Vec<Item> {
        match fs::read_dir(&self.current_path) {
            Ok(readDir) => {
                readDir.map(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let name = entry.file_name().into_string().unwrap();
                    let metadata = fs::metadata(&path).unwrap();
                    let size = metadata.len();
                    let modified = metadata.modified().unwrap();
                    let modified_dt = Self::system_time_to_date_time(modified);
                    let item_type = if metadata.is_dir() {
                        ItemType::Directory
                    } else {
                        ItemType::File
                    };
                    Item {
                        name,
                        path,
                        selected: false,
                        item_type,
                        size,
                        modified: modified_dt,
                    }
                }).collect()
            },
            Err(e) => { panic!("Readdir panic, #{}", e.to_string()) }
        }
    }

    fn system_time_to_date_time(system_time: SystemTime) -> DateTime<Local> {
        system_time.into()
    }
}

impl Default for Navigator {
    fn default() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let path = home_dir.join("commander-tmp");
        Self::new(path)
    }
}