mod add;
mod block;
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
