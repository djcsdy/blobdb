use crate::cli::options::AddOptions;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{copy, Error};

pub fn add(options: AddOptions) -> Result<[u8; 32], Error> {
    let mut file = File::open(&options.path)?;
    let mut hasher = Sha256::new();
    copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    Ok(hash.into())
}
