#![feature(let_chains)]

mod assets;
mod cli;
mod collections;
mod commands;
mod front_matter;
mod has_extension;
mod minifier;
mod register_templates;
mod render;

use crate::cli::{Cli, Commands};
use clap::Parser;

use crate::commands::build::build;
use serde::Serialize;

#[derive(Serialize)]
struct CollectionEntry {
    link: String,
    name: String,
}

fn main() -> Result<(), ()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        None => Err(()),
        Some(Commands::Build { input, output }) => Ok(build(input, output)),
        _ => Err(()),
    }
}
