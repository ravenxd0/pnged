use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pnged")]
#[command(about = "Hiding Message in PNG",long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode PNG with Mesaage
    #[command(arg_required_else_help = true)]
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        #[arg(required = false)]
        output_file: Option<String>,
    },

    /// Decode Message From PNG
    #[command(arg_required_else_help = true)]
    Decode { path: PathBuf, chunk_type: String },

    /// Remove Message From PNG
    #[command(arg_required_else_help = true)]
    Remove { path: PathBuf, chunk_type: String },

    /// Print Message from PNG
    #[command(arg_required_else_help = true)]
    Print { path: PathBuf },
}
