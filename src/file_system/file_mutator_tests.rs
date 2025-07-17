#[cfg(test)]
mod tests {
    use std::{fs, io, io::Write, path::PathBuf};
    use tempfile::tempdir;
    use crate::file_system::file_mutator::FileMutator;

    fn create_file(path: &PathBuf, contents: &str) -> io::Result<()> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;
        f.write_all(contents.as_bytes())?;
        f.sync_all()
    }

    #[test]
    fn durable_move_basic() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("foo.txt");
        let dst_path = dir.path().join("bar.txt");

        create_file(&src_path, "hello world").unwrap();

        FileMutator::durable_move(&src_path, &dst_path).unwrap();

        assert!(!src_path.exists(), "source file should be gone");
        assert!(dst_path.exists(), "destination file should exist");

        let data = fs::read_to_string(&dst_path).unwrap();
        assert_eq!(data, "hello world");
    }

    #[test]
    fn durable_move_same_directory_rename() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.txt");
        let dst = dir.path().join("a-renamed.txt");

        create_file(&src, "content").unwrap();
        FileMutator::durable_move(&src, &dst).unwrap();

        assert!(!src.exists());
        assert!(dst.exists());
        assert_eq!(fs::read_to_string(&dst).unwrap(), "content");
    }

    #[test]
    fn durable_move_nonexistent_source_errors() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("no-such-file.txt");
        let dst = dir.path().join("whatever.txt");

        let err = FileMutator::durable_move(&src, &dst).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn durable_move_overwrite_dst() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("orig.txt");
        let dst = dir.path().join("dup.txt");

        create_file(&src, "first").unwrap();
        create_file(&dst, "second").unwrap();

        let res = FileMutator::durable_move(&src, &dst);

        {
            res.unwrap();
            assert!(!src.exists());
            assert_eq!(fs::read_to_string(&dst).unwrap(), "first");
        }
    }
}