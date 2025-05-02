#!/bin/zsh

# =============================================================================
# Helper Functions
# =============================================================================


# Function to print an error message.
alias print_err=">&2 echo Error: "

# Gets a string representation for the kind of repository we are in.
#
#   Git repos print:       "git"
#   Graphite prints:       "graphite"
#   Anything else prints:  "none"
#
function vcs_kind() {
  GIT_DIR=$(git rev-parse --git-dir 2> /dev/null)
  if [ -n "$GIT_DIR" ]; then
    GRAPHITE_CONFIG="$GIT_DIR/.graphite_repo_config"
    if [[ -f "$GRAPHITE_CONFIG" ]]; then
      echo "graphite"
      return 1
    else
      echo "git"
      return 1
    fi
  fi
  echo "none"
}

# Prints a generic error message for an unknown vcs then exits.
#
# Note: Always return 1 after calling this function so the method will break
# logical chains properly. (TODO: Can this be done automatically somehow?)
function vcs_none() {
  print_err "$(pwd) is not managed by a known version control system"
  return 1
}

# =============================================================================
# Commands
# =============================================================================

# Prints the current status of the repository.
#
# Accepts no arguments.
function st() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git status
      return 0
      ;;
    graphite)
      git status
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints detailed current status of the repository.
#
# Accepts no arguments.
function stt() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git status
      return 0
      ;;
    graphite)
      # TODO: -n 0 doesn't work, figure out how to only show things starting
      # with ◉, and then stopping output when you see ◯ (without losing color).
      gt log -n 1
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints a short graph of the current repository's commit structure. Alias
# to force the git version even in graphite.
function gsl() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git log \
      -10 \
      --graph \
      --abbrev-commit \
      --decorate \
      --format=format:'%C(bold blue)%h%C(reset) %C(bold green)(%ar)%C(reset)%C(auto)%d%C(reset)%n''          %C(white)%s%C(reset)%n''          %C(dim white)- %an%C(reset)'
      return 0
      ;;
    graphite)
      git log \
      -10 \
      --graph \
      --abbrev-commit \
      --decorate \
      --format=format:'%C(bold blue)%h%C(reset) %C(bold green)(%ar)%C(reset)%C(auto)%d%C(reset)%n''          %C(white)%s%C(reset)%n''          %C(dim white)- %an%C(reset)'
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints a short graph of the current repository's commit structure.
#
# Accepts no arguments.
function sl() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git log \
      -10 \
      --graph \
      --abbrev-commit \
      --decorate \
      --format=format:'%C(bold blue)%h%C(reset) %C(bold green)(%ar)%C(reset)%C(auto)%d%C(reset)%n''          %C(white)%s%C(reset)%n''          %C(dim white)- %an%C(reset)'
      return 0
      ;;
    graphite)
      gt log short
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints a more detailed graph of the current repository's commit structure.
#
# Accepts no arguments.
function sll() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      # Git has no special handling here, reuse sl.
      sl
      return 0
      ;;
    graphite)
      gt log -n 2
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Checkout a particular place in the repository.
#
# Passes all arguments to underlying command.
function co() {
  case $(vcs_kind) in
    git)
      git checkout "$@"
      return 0
      ;;
    graphite)
      git checkout "$@"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Submit Stack: Submits all changes in the stack.
function ss() {
  case $(vcs_kind) in
    git)
      print_err "Cannot submit diffs from git repos, only graphite."
      return 1
      ;;
    graphite)
      gt submit
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Create a new commit with the given message.
#
# Accepts a single string argument that is the message.
function cm() {
  if [ -z "$1" ]; then
    print_err "Must provide a single message argument, no arguments provided."
    return 1
  fi

  if [ -n "$2" ]; then
    print_err "Must provide a single message argument, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      # Have to git add first in order to capture untracked files.
      git add -A && git commit -am "$1"
      return 0
      ;;
    graphite)
      git add -A && git commit -am "$1"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Ammends the commit with the current changes.
function ca() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git add -A && git commit --amend --no-edit
      return 0
      ;;
    graphite)
      git add -A && git commit --amend --no-edit
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Creates a new graphite branch with the given branch name and message.
function cn() {
  case $(vcs_kind) in
    git)
      print_err "Not implemented for plain git repos."
      return 1
      ;;
    graphite)
      graphite_cn "$@"
      # Return whatever the last function did
      return $?
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Helper function for cn that has already ensured we're in a graphite repo.
function graphite_cn() {
  BRANCH=""
  MESSAGE=""

  # No arguments provided.
  if [ -z "$1" ]; then
    print_err "Expecting branch or message as first argument, no arguments provided."
    return 1
  fi

  # Only one argument provided.
  if [ -z "$2" ]; then
    # Check if the current branch ends in a digit.
    CURR_BRANCH=$(graphite_curr_branch)
    CURR_BRANCH_ENDS_IN_DIGIT="$(echo "$CURR_BRANCH" | grep -E '[0-9]$')"
    if [ -z "$CURR_BRANCH_ENDS_IN_DIGIT" ]; then
      print_err "Current branch does not end in a digit, cannot infer next branch name."
      print_err "Please provide both a branch name and message arguments."
      return 1
    fi

    # Increment the branch by 1.
    NEXT_BRANCH_NAME="$(echo "$CURR_BRANCH" | perl -pe 's/(\d+)$/$1+1/e')"

    # Save the inferred branch and messages.
    BRANCH="$NEXT_BRANCH_NAME"
    MESSAGE="$1"
  else
    # At least two arguments provided here.
    BRANCH="$1"
    MESSAGE="$2"
  fi

  # More than two arguments provided.
  if [ -n "$3" ]; then
    print_err "Expecting branch or message as first argument, and at most 2 arguments. Too many arguments provided."
    return 1
  fi

  # Check if the message looks like a conventional commit message. That means
  # it starts with a word character, can have optional parenthesis, and ends
  # with a colon.
  if ! echo "$MESSAGE" | grep -qE '^[a-zA-Z0-9_]+(\([a-zA-Z0-9_]+\))?:'; then
    print_err "Message does not look like a conventional commit message, please use a message like 'feat: description'"
    return 1
  fi

  # Check if the git repo is clean, if so then they probably don't mean to
  # create a new branch and commit just yet.
  if [ -z "$(git status --porcelain)" ]; then
    print_err "Git repo has no changes, prefer to make changes before creating a new branch via graphite."
    return 1
  fi

  # Create the new branch and message for graphite.
  gt create "$BRANCH" -m "$MESSAGE" --all
}

# Gets the current commit hash in the repository.
#
# Accepts no arguments.
function curr_hash() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  case $(vcs_kind) in
    git)
      git log -1 --pretty=format:"%h"
      return 0
      ;;
    graphite)
      git log -1 --pretty=format:"%h"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Gets the current branch in a graphite repo.
#
# Accepts no arguments.
function graphite_curr_branch() {
  if [ -n "$1" ]; then
    print_err "There should be no arguments, too many arguments provided."
    return 1
  fi

  git branch --show-current
  return $?
}
