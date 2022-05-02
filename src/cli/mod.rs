mod add;
mod options;
mod read;

use add::add;
use clap::Parser;
use options::{Options, Subcommand};
use read::read;

pub fn main() {
    let options = Options::parse();

    match options.subcommand {
        Subcommand::Add(options) => {
            add(options);
        }
        Subcommand::Read(options) => {
            read(options);
        }
    }
}
