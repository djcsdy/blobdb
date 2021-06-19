mod add;
mod options;

use crate::options::{Options, Subcommand};
use clap::Clap;

fn main() {
    let options = Options::parse();

    match options.subcommand {
        Subcommand::Add(_) => {
            panic!("TODO: Add");
        }
    }
}
