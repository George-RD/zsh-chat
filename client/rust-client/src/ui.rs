use ratatui::{
    widgets::{Block, Borders, List, ListItem, Paragraph},
    style::{Color, Modifier, Style},
    layout::{Layout, Constraint, Direction},
    Frame,
};
use crate::models::Post;

pub fn render_feed(f: &mut Frame, posts: &[Post]) {
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
}
