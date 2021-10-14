use crate::cli::options::AddOptions;
use crate::lib::{BlobId, Db};
use std::fs::File;
use std::io::Result;
use std::path::Path;

pub fn add(options: AddOptions) -> Result<BlobId> {
    let db = Db::open(Path::new("."))?;
    let mut file = File::open(&options.path)?;
    db.import_blob(&mut file)
}
