use std::path::{Path, PathBuf};

pub const ROOT_FILE_NAME: &str = "BlobDb";

pub fn root_file_path(base_path: &Path) -> PathBuf {
    base_path.join(ROOT_FILE_NAME)
}

pub fn lock_file_path(base_path: &Path) -> PathBuf {
    base_path.join("BlobDb.lock")
}

pub fn temp_dir_path(base_path: &Path) -> PathBuf {
    base_path.join("tmp")
}
