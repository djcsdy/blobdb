use crate::blob::BlobId;
use hex;
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

pub fn blob_file_path(base_path: &Path, blob_id: BlobId) -> PathBuf {
    blob_id
        .0
        .iter()
        .map(|b| hex::encode([*b]))
        .fold(base_path.into(), |path, segment| path.join(segment))
}
