mod command;
mod error;
mod examples;

mod mytracer;
pub use error::{Error, Result};

use clap::Parser;
use command::{Args, Command, ExCase};

use crate::mytracer::setup_simple_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    setup_simple_tracing();

    let args = Args::parse();
    match args.cmd {
        Command::Ex01 => {
            let _ = examples::ex01_run().await?;
        }
        Command::Ex02 { case } => match case {
            ExCase::Case01 { .. } => {
                todo!()
            }
            ExCase::Case02 => {
                todo!()
            }
        },
        Command::Ex03 { case: _case } => {
            todo!()
        }
    }

    Ok(())
}
