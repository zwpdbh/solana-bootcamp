mod command;
mod error;
mod examples;

mod mytracer;
pub use error::{Error, Result};

use clap::Parser;
use command::{Args, Command};

use crate::mytracer::setup_simple_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    setup_simple_tracing();

    let args = Args::parse();
    match args.cmd {
        Command::Ex01 => {
            let _ = examples::ex01_run().await?;
        }
        Command::Ex02 => {
            let _ = examples::ex02_run().await?;
        }
    }

    Ok(())
}
