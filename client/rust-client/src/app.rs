use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;
use crate::api::ApiClient;
use crate::ui;

pub async fn run_tui_feed(api_client: &ApiClient) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut posts = api_client.fetch_posts().await?;
    let mut last_fetch = std::time::Instant::now();

    loop {
        terminal.draw(|f| {
            ui::render_feed(f, &posts);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        posts = api_client.fetch_posts().await?;
                        last_fetch = std::time::Instant::now();
                    }
                    _ => {}
                }
            }
        }

        // Auto-refresh every 30 seconds
        if last_fetch.elapsed() > Duration::from_secs(30) {
            posts = api_client.fetch_posts().await?;
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
