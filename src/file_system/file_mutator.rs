use std::path::Path;
use std::{fs, io};

pub struct FileMutator;

impl FileMutator {
    pub fn durable_move<T: AsRef<Path>, A: AsRef<Path>>(src: T, dst: A) -> io::Result<()> {
        // open & sync the source file
        let src = src.as_ref();
        fs::OpenOptions::new().read(true).open(src)?.sync_all()?;

        let src_dir = src.parent().unwrap();
        let dst = dst.as_ref();
        let dst_dir = dst.parent().unwrap();

        fs::rename(src, dst)?;

        fs::File::open(src_dir)?.sync_all()?;
        if src_dir != dst_dir {
            fs::File::open(dst_dir)?.sync_all()?;
        }
        Ok(())
    }
}