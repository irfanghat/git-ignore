#![allow(unused)]

mod cli;
mod commands;
mod git;
mod ignore;
mod model;
mod sensitive;
mod templates;

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_templates.rs"));
}

use generated::TEMPLATES;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli.run()
}
