use std::path::PathBuf;

pub struct Params {
    pub(crate) left_path: PathBuf,
    pub(crate) right_path: PathBuf,
    pub(crate) scale: f32,
}