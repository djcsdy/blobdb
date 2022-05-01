use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Parser)]
pub enum Subcommand {
    Add(AddOptions),
}

#[derive(Parser)]
pub struct AddOptions {
    pub path: String,
}
