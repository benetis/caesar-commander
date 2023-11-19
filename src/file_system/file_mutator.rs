use std::io;
use std::path::PathBuf;

pub struct FileMutator {}

impl FileMutator {
    pub fn new() -> Self {
        FileMutator {}
    }

    fn move_file(file_path: &PathBuf, destination_path: &PathBuf) -> io::Result<()> {
        std::fs::rename(file_path, destination_path)
    }
}