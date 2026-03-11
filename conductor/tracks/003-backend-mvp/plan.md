# Track: 003-backend-mvp Plan

## Objective
Implement a basic Node.js (Express) backend to handle user posts and provide a JSON feed API.

## Plan
1.  [x] Initialize a Node.js project in the `server/` directory.
2.  [x] Install necessary dependencies (`express`, `cors`, `body-parser`, `dotenv`).
3.  [x] Create a basic Express server at `server/index.js`.
4.  [x] Implement the following endpoints:
    -   `GET /api/posts`: Return a JSON list of recent posts.
    -   `POST /api/posts`: Accept a JSON object containing a post message.
5.  [x] Use an in-memory array for post storage (to keep it simple for MVP).
6.  [x] Test the backend using `curl` or a similar tool.
7.  [x] Update the CLI (`client/zsh-chat.zsh`) to interact with the real backend.
8.  [x] Commit the backend implementation.
