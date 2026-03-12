# zsh-chat Ethos & Coding Standards

## 1. Lo-Fi High-Tech Aesthetic
- **4-Bit Constraint:** Images and emojis should be downsampled to 4-bit (16 colors) or 2-bit (4 colors). This is both a stylistic choice and a content-safety feature.
- **Terminal First:** Do not try to emulate a browser. Use ASCII/Unicode characters, borders, and ANSI colors.
- **Retro-Dithering:** When converting images, use dithering algorithms to maintain "texture" even at low bit-depths.

## 2. Progressive Disclosure
- **Level 1 (The Vibe Coder):** A beautiful, simple TUI (`ratatui`) that feels like a cozy chat app.
- **Level 2 (The Power User):** Keybindings for navigation, tmux pane integration, and custom themes.
- **Level 3 (The Wizard):** Pipe-friendly CLI commands (`| zsh-chat post`) and local-first cryptographic identity management.

## 3. Decentralized & Sovereign
- **No Central DB:** Identity is owned by the user (Private Key). Messages are hosted by Relays.
- **Relay Agnostic:** The client should be able to connect to multiple relays simultaneously.
- **Verification:** Every message MUST be cryptographically signed. Unsigned messages are discarded.

## 4. Performance & Portability
- **Static Binaries:** The project must remain "Full Rust" to ensure a single-binary install.
- **Minimal Dependencies:** Prefer small, focused crates. Avoid "heavy" web frameworks.
- **Tmux Integration:** Support for `TMUX` environment detection to allow status-bar notifications or automatic pane splitting.
