mod models;
mod api;
mod ui;
mod app;
mod relay;

use clap::{Parser, Subcommand};
use crate::models::Post;
use tokio_tungstenite::connect_async;
use futures_util::SinkExt;

#[derive(Parser)]
#[command(name = "zsh-chat")]
#[command(about = "A terminal-based social media platform (Rust version)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, env = "ZSH_CHAT_API_URL", default_value = "ws://localhost:8080")]
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
    /// Run a local relay server
    Serve {
        /// The address to bind to
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
    /// View or edit your profile settings
    Profile,
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

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
                timestamp: Some(chrono::Local::now()),
            };

            let (mut ws_stream, _) = connect_async(&cli.api_url).await?;
            let msg = serde_json::to_string(&new_post)?;
            ws_sender_send(&mut ws_stream, msg).await?;

            println!("Successfully posted!");
        }
        Commands::Feed => {
            app::run_tui_feed(&cli.api_url).await?;
        }
        Commands::Serve { addr } => {
            relay::run_relay(addr).await?;
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

async fn ws_sender_send(ws_stream: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, msg: String) -> Result<(), Box<dyn std::error::Error>> {
    ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await?;
    Ok(())
}
