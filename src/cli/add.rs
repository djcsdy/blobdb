use crate::cli::options::AddOptions;
use crate::lib::{BlobId, Db};
use std::fs::File;
use std::path::Path;

pub fn add(options: AddOptions) -> () {
    let db = Db::open(Path::new(".")).unwrap();
    let mut file = File::open(&options.path).unwrap();
    let blob_id = db.import_blob(&mut file).unwrap();
    println!("{}", &blob_id);
}
