use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Ex01,
    Ex02,
}
