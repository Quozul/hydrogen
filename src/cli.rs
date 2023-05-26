use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Build the site
    Build {
        /// Input directory
        #[arg(short, long, default_value = ".")]
        input: PathBuf,

        /// Build output
        #[arg(short, long, default_value = "_site")]
        output: PathBuf,
    },
    /// Start a server with server-side rendering for development
    Dev {},
    /// Build and start a production server
    Serve {},
    /// Create a projects with default structure and files
    Init {},
}
