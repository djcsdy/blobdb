use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Parser)]
pub enum Subcommand {
    Add(AddOptions),
    Read(ReadOptions),
}

#[derive(Parser)]
pub struct AddOptions {
    pub path: String,
}

#[derive(Parser)]
pub struct ReadOptions {
    pub blob_id: String,
    pub path: String,
}
