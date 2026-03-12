# Track: 003-backend-mvp Plan (RETIRED/REPLACED)

## Objective
Develop a simple Node.js backend for authentication and post storage.

## Status: Replaced by Rust Relay Architecture
The Node.js backend has been nuked in favor of a "Full Rust" single-binary architecture.

## New Plan (Issue #5)
1.  [x] Nuke the `server/` Node.js directory.
2.  [x] Implement a Rust-based WebSocket relay in the `zsh-chat` binary.
3.  [x] Update the Rust client to use WebSockets for real-time communication.
4.  [ ] Implement cryptographic signature verification (Issue #6).
5.  [ ] Add persistent storage for the relay (currently in-memory).
