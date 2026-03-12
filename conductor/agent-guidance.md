# Agent Guidance: The Evolutionary Engine

## 1. The Core Directives
- **Read First:** Every Ralph loop iteration MUST start by reading `conductor/vision.md` and `conductor/product.md`.
- **Evolve Second:** Before starting a task, check if the "Mission" needs updating based on previous development or user feedback.
- **Task Management:** Use GitHub Issues as the primary source of truth for "What to do next."
    - If a task is complete: Close the issue.
    - If a new idea or refinement is discovered: Create a new issue and label it accordingly.

## 2. Technical Philosophy (The "Lo-Fi High-Tech" Stack)
- **Full Rust:** Single-binary distribution (client + relay).
- **Identity:** Ed25519 cryptographic keypairs (no passwords).
- **Aesthetic:** Static emoji mapping to ANSI and dynamic 4-bit/2-bit image downsampling.
- **Social Physics:** The Fibonacci Feed weighting ($\phi \approx 1.618$).

## 3. The Ralph Loop Strategy
- **Socratic Refactoring:** Constantly simplify. "Is this Node.js heritage? Can we do it better in Rust?"
- **Validation:** Every change requires a unit test or a verified reproduction script.
- **Completion:** The "Completion Promise" is ONLY valid when all GitHub issues are resolved, the README is fully ticked, and no further structural improvements are visible.

## 4. Environment Specifics
- **CARGO_HOME**: Always set to project-local tmp for macOS Seatbelt compatibility.
- **Tmux Integration**: Detect and leverage the `TMUX` environment where possible.
