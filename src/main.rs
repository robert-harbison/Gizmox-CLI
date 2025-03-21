mod commands;

use clap::{Parser, Subcommand};
use commands::copy_rename::run_copy_rename;

#[derive(Parser)]
#[command(name = "gm")]
#[command(about = "A CLI with useful utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Creates multiple renamed copies of a file.")]
    CopyRename {
        /// Input file to copy
        #[arg(help = "The single input file to copy")]
        input: String,

        /// Optional full extension override (e.g. "png.meta")
        #[arg(long)]
        ext: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CopyRename { input, ext } => run_copy_rename(input, ext),
    }
}
