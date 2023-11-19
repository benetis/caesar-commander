use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::model::*;

#[derive(Clone)]
pub struct Navigator {
    current_path: PathBuf,
}

impl Navigator {
    pub fn new(initial_path: &PathBuf) -> Self {
        Navigator { current_path: initial_path.clone() }
    }

    pub fn open_dir(&mut self, path: &PathBuf) {
        self.current_path = path.clone();
    }

    pub fn list_contents(&self) -> Vec<Item> {
        match fs::read_dir(&self.current_path) {
            Ok(read_dir) => {
                read_dir.map(|entry| {
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
            }
            Err(e) => { panic!("Readdir panic, #{}", e.to_string()) }
        }
    }

    pub fn go_up(&mut self) {
        let parent = self.current_path.parent();

        match parent {
            Some(parent) => {
                self.current_path = parent.to_path_buf();
            },
            None => {
                self.current_path = PathBuf::from("/");
            }
        }
    }

    pub fn breadcrumbs(&self) -> Vec<String> {
        self.current_path.iter().skip(1)
            .map(|os_str| os_str.to_str().unwrap().to_string())
            .collect()
    }

    fn system_time_to_date_time(system_time: SystemTime) -> DateTime<Local> {
        system_time.into()
    }
}