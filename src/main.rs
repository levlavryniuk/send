use clap::Parser;

use cli::Cli;

use crate::cli::Commands;

mod cli;
mod ls;
mod recv;
mod sender;
#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Listen {} => {
            println!("Starting reciever...");
            recv::activate_reciever().await.unwrap();
        }
        Commands::Send { file_name, to } => sender::send(&file_name, &to).await.unwrap(),
        Commands::Ls => ls::list_recievers().await.unwrap(),
    }
}
