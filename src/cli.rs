use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Listen {},
    Send { file_name: String, to: String },
    Ls,
}
