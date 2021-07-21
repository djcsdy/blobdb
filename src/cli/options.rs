use clap::Clap;

#[derive(Clap)]
pub struct Options {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Clap)]
pub enum Subcommand {
    Add(AddOptions),
}

#[derive(Clap)]
pub struct AddOptions {
    pub path: String,
}
