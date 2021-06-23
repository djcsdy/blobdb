mod add;
mod blob_id;
mod block;
mod db_id;
mod options;
mod packet;

use crate::add::add;
use crate::options::{Options, Subcommand};
use clap::Clap;

fn main() {
    let options = Options::parse();

    match options.subcommand {
        Subcommand::Add(options) => {
            add(options).unwrap();
        }
    }
}
