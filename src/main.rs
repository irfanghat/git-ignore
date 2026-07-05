#![allow(unused)]

mod cli;
mod commands;
mod git;
mod ignore;
mod model;
mod sensitive;
mod templates;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.run()
}
