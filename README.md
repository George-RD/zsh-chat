# zsh-chat

A terminal-based social media platform, built for the shell.

## Vision
zsh-chat aims to bring social interaction directly to your command line, allowing you to share snippets, thoughts, and connect with other developers without leaving your workflow.

## Features (Planned)
- [ ] Post updates from the terminal.
- [ ] View a feed of other users' posts.
- [ ] Reply and interact with posts.
- [ ] Profile management via local config.
- [ ] Plugin architecture for extensibility.

## Project Structure
- `client/`: The terminal client.
    - `rust-client/`: The new Rust-based client (WIP).
    - `zsh-chat.zsh`: The original Zsh-based client.
- `server/`: A simple Node.js (Express) backend.
- `conductor/`: Project management and track plans.

## Tech Stack
- **Frontend**: Rust (Primary client), Zsh (Legacy/Helper scripts).
- **Backend**: Node.js (Express) MVP.
- **TUI**: `ratatui` or `crossterm` (planned for Rust).
- **Project Tracking**: Conductor Project Framework.

## Getting Started

### Backend
1. `cd server`
2. `npm install`
3. `npm start`

### Rust Client
1. Ensure you have Rust installed.
2. `cd client/rust-client`
3. `cargo run -- feed`
