use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use chrono::{DateTime, Local};

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

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    author: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Local>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = Client::new();

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

            let res = client
                .post(format!("{}/posts", cli.api_url))
                .json(&new_post)
                .send()
                .await?;

            if res.status().is_success() {
                println!("Successfully posted!");
            } else {
                eprintln!("Failed to post: {}", res.status());
            }
        }
        Commands::Feed => {
            let posts: Vec<Post> = client
                .get(format!("{}/posts", cli.api_url))
                .send()
                .await?
                .json()
                .await?;

            println!("\n--- Community Feed ---");
            for post in posts {
                let time_str = post.timestamp
                    .map(|t| t.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                println!("[{}] @{}: {}", time_str, post.author, post.message);
            }
            println!("----------------------\n");
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
