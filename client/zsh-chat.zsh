#!/usr/bin/env zsh

# --- Strict Mode & Options ---
setopt errexit nounset pipefail

# --- Global Constants ---
readonly VERSION="0.1.0"
readonly SCRIPT_NAME="${0:t}"

# --- Utility Functions ---
log_info() {
  print "[\e[34mINFO\e[0m] $1"
}

log_error() {
  print -u2 "[\e[31mERROR\e[0m] $1"
}

usage() {
  print "zsh-chat v$VERSION"
  print "A terminal-based social media platform."
  print ""
  print "Usage: $SCRIPT_NAME <command> [options]"
  print ""
  print "Commands:"
  print "  post <message>  Post a new update to the feed."
  print "  feed            View the latest posts from the community."
  print "  profile         View or edit your profile settings."
  print "  version         Show version information."
  print ""
  print "Options:"
  print "  -h, --help      Show this help message."
}

# --- Command Handlers ---
cmd_post() {
  if (( $# == 0 )); then
    log_error "Post message cannot be empty."
    exit 1
  fi
  local message="$*"
  log_info "Attempting to post: '$message'..."
  # Mocking API call
  print "\e[32mSuccessfully posted!\e[0m"
}

cmd_feed() {
  log_info "Fetching latest posts..."
  # Mocking feed output
  print ""
  print -- "--- Community Feed ---"
  print "@george: Just started building zsh-chat! 🚀"
  print "@alice: The terminal is the best social media platform."
  print "@bob: Is this thing on?"
  print -- "----------------------"
  print ""
}

cmd_profile() {
  log_info "Profile settings (Mocked):"
  print "Username: @user"
  print "Joined: March 2026"
}

# --- Main Execution ---
main() {
  if (( $# == 0 )); then
    usage
    exit 0
  fi

  local command="$1"
  shift

  case "$command" in
    post)
      cmd_post "$@"
      ;;
    feed)
      cmd_feed "$@"
      ;;
    profile)
      cmd_profile "$@"
      ;;
    version)
      print "zsh-chat v$VERSION"
      ;;
    -h|--help)
      usage
      ;;
    *)
      log_error "Unknown command: $command"
      usage
      exit 1
      ;;
  esac
}

main "$@"
