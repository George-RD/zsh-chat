use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use chrono::{DateTime, Local};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Color, Modifier, Style},
    layout::{Layout, Constraint, Direction},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            run_tui_feed(&cli.api_url, &client).await?;
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

async fn run_tui_feed(api_url: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut posts = fetch_posts(api_url, client).await?;
    let mut last_fetch = std::time::Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let header = Paragraph::new("zsh-chat Community Feed")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = posts
                .iter()
                .map(|p| {
                    let time_str = p.timestamp
                        .map(|t| t.format("%H:%M:%S").to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    ListItem::new(format!("[{}] @{}: {}", time_str, p.author, p.message))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Feed"))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">> ");
            f.render_widget(list, chunks[1]);

            let footer = Paragraph::new("Press 'q' to quit, 'r' to refresh")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title("Footer"));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        posts = fetch_posts(api_url, client).await?;
                        last_fetch = std::time::Instant::now();
                    }
                    _ => {}
                }
            }
        }

        // Auto-refresh every 30 seconds
        if last_fetch.elapsed() > Duration::from_secs(30) {
            posts = fetch_posts(api_url, client).await?;
            last_fetch = std::time::Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn fetch_posts(api_url: &str, client: &Client) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let posts: Vec<Post> = client
        .get(format!("{}/posts", api_url))
        .send()
        .await?
        .json()
        .await?;
    Ok(posts)
}
