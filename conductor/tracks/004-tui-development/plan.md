# Track: 004-tui-development Plan

## Objective
Enhance the CLI feed view with interactive TUI components for a more "delightful" experience.

## Plan
1.  [ ] Research TUI frameworks or libraries compatible with Zsh (e.g., `dialog`, `whiptail`, or building a custom pager).
2.  [ ] Implement a custom "pager" in zsh to display the feed with:
    -   Basic scrolling support.
    -   Colored headers and highlights for usernames.
    -   Keybindings (e.g., 'r' to refresh, 'q' to quit, 'p' to post).
3.  [ ] Integrate the TUI view into `client/zsh-chat.zsh feed`.
4.  [ ] Add a "loading" indicator when fetching the feed.
5.  [ ] Test the interactive feed.
6.  [ ] Commit the TUI enhancements.
