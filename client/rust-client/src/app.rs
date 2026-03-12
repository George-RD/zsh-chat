use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use crate::ui;
use crate::models::Post;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;

pub async fn run_tui_feed(api_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut posts: Vec<Post> = Vec::new();

    // Connect to WebSocket relay
    let (ws_stream, _) = connect_async(api_url).await?;
    let (_, mut ws_receiver) = ws_stream.split();

    loop {
        terminal.draw(|f| {
            ui::render_feed(f, &posts);
        })?;

        // Check for WebSocket messages
        tokio::select! {
            Some(Ok(msg)) = ws_receiver.next() => {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    if let Ok(post) = serde_json::from_str::<Post>(&text) {
                        posts.push(post);
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(50)) => {}
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
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
