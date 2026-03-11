mod models;
mod api;
mod ui;
mod app;

use clap::{Parser, Subcommand};
use crate::models::Post;
use crate::api::ApiClient;

#[derive(Parser)]
#[command(name = "zsh-chat")]
#[command(about = "A terminal-based social media platform (Rust version)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, env = "ZSH_CHAT_API_URL", default_value = "http://localhost:3000/api")]
    api_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Post a new update to the feed
    Post {
        /// The message to post
        message: Vec<String>,
    },
    /// View the latest posts from the community
    Feed,
    /// View or edit your profile settings
    Profile,
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let api_client = ApiClient::new(cli.api_url);

    match &cli.command {
        Commands::Post { message } => {
            let message = message.join(" ");
            if message.is_empty() {
                eprintln!("Error: Post message cannot be empty.");
                std::process::exit(1);
            }

            let author = std::env::var("USER").unwrap_or_else(|_| "anonymous".to_string());
            let new_post = Post {
                id: None,
                author,
                message,
                timestamp: None,
            };

            api_client.create_post(&new_post).await?;
            println!("Successfully posted!");
        }
        Commands::Feed => {
            app::run_tui_feed(&api_client).await?;
        }
        Commands::Profile => {
            println!("Profile settings (Mocked):");
            println!("Username: @{}", std::env::var("USER").unwrap_or_else(|_| "user".to_string()));
            println!("Joined: March 2026");
        }
        Commands::Version => {
            println!("zsh-chat v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
