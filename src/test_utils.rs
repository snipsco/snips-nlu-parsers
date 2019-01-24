use std::path::{Path, PathBuf};

pub fn test_path() -> PathBuf {
    Path::new("data").join("tests")
}
