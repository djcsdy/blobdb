use crate::cli::options::ReadOptions;
use crate::lib::{BlobId, Db, FsDb};
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn read(options: ReadOptions) {
    let db = FsDb::open(Path::new(".")).unwrap();
    let blob_id = BlobId::parse(&options.blob_id).unwrap();
    let mut reader = db.read_blob(blob_id).unwrap();
    let mut file = File::options().write(true).open(&options.path).unwrap();
    copy(&mut reader, &mut file).unwrap();
}
