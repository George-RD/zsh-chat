# Tech Stack: zsh-chat

## Core Technologies
- **Shell**: Zsh (Primary runtime for the client).
- **TUI Framework**: `gum` (by Charm) for rich interactive terminal experiences.
- **Data Exchange**: JSON (over HTTPS/WSS).

## Potential Backend Options
1.  **Centralized (Node.js/Go)**: Simplest for MVP, allows for easy user management and feed aggregation.
2.  **Decentralized (AT Protocol / Nostr)**: Highly aligned with terminal culture, provides data ownership and interoperability.
    - *Decision for MVP*: We will start with a simple **Node.js (Express)** or **Python (FastAPI)** backend to iterate quickly on the core user experience.

## Infrastructure
- **Hosting**: Vercel or Railway for the backend.
- **Persistence**: PostgreSQL or MongoDB for feed storage.

## Tools
- **Version Control**: Git / GitHub.
- **Project Tracking**: GitHub Issues.
- **Development**: Gemini CLI for autonomous engineering.
