mod add;
mod options;

use add::add;
use clap::Clap;
use options::{Options, Subcommand};

pub fn main() {
    let options = Options::parse();

    match options.subcommand {
        Subcommand::Add(options) => {
            add(options).unwrap();
        }
    }
}
