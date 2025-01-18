#!/bin/zsh

### My old commands for version control, will update __eventually__ ###

# Function to print an error message.
alias print_err=">&2 echo Error: "

# Gets a string representation for the kind of repository we are in.
#
#   Git repos print:       "git"
#   Mercurial repos print: "hg"
#   Anything else prints:  "none"
#
function vcs_kind() {
  IS_IN_GIT=$(git rev-parse --git-dir 2> /dev/null)
  if [ $IS_IN_GIT ]; then
    echo "git"
    return 1
  fi
  IS_IN_MERCURIAL=$(hg root 2> /dev/null)
  if [ $IS_IN_MERCURIAL ]; then
    echo "hg"
    return 1
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

# Function to check if in a mercurial repository.
#
# Arugments are ignored.
function is_hg() {
  case $(vcs_kind) in
    git)
      print_err "This is a git repository, expected mercurial."
      return 1
      ;;
    hg)
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
    hg)
      hg commit -Am "$1"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Ammends the commit with the current changes.
#
# Accepts no arguments.
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
    hg)
      hg commit --amend --rebase -Am "`hg log -l 1 --template "{desc}"`"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

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
    hg)
      hg status
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints a graph of the current repository's commit structure. Only printing
# the current path to master if possible.
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
    hg)
      hg --config extensions.pager=! ssl --all -r top%master
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Prints a graph of the current repository's commit structure. Prints all
# commits.
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
    hg)
      hg --config ui.paginate=never ssl
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Go to a particular place in the repository.
#
# Passes all arguments to underlying command.
function go() {
  case $(vcs_kind) in
    git)
      git checkout "$@"
      return 0
      ;;
    hg)
      hg checkout "$@"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
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
    hg)
      hg log -l 1 --template "{node|short}"
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}

# Submits a diff. Only works in mercurial.
#
# If no arguments are specified it submits the currently active commit,
# otherwise all arguments are passed to the underlying command.
function s() {
  case $(vcs_kind) in
    git)
      print_err "Cannot submit diffs from git repos, only www."
      return 1
      ;;
    hg)
      if [[ -z "$1" ]]; then
        jf s --prepare $(curr_hash)
      else
        jf s --prepare "$@"
      fi
      return 0
      ;;
    *)
      vcs_none
      return 1
      ;;
  esac
}
