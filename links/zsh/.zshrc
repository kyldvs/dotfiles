# =============================================================================
# Powerlevel10k instant prompt.
# =============================================================================

# Enable Powerlevel10k instant prompt. Should stay close to the top of ~/.zshrc.
# Initialization code that may require console input (password prompts, [y/n]
# confirmations, etc.) must go above this block; everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

# =============================================================================
# Homebrew (package manager).
# =============================================================================

if [[ -f "/opt/homebrew/bin/brew" ]] then
  # If you're using macOS, you'll want this enabled.
  eval "$(/opt/homebrew/bin/brew shellenv)"
fi

PATH="/opt/homebrew/bin:$PATH"

# =============================================================================
# zinit (zsh plugin manager).
# =============================================================================

# Set the directory to store zinit and plugins.
ZINIT_HOME="${XDG_DATA_HOME:-${HOME}/.local/share}/zinit/zinit.git"

# Download Zinit, if it's not there yet.
if [ ! -d "$ZINIT_HOME" ]; then
  mkdir -p "$(dirname $ZINIT_HOME)"
  git clone https://github.com/zdharma-continuum/zinit.git "$ZINIT_HOME"
fi

# Load zinit.
source "${ZINIT_HOME}/zinit.zsh"

# Add in Powerlevel10k.
zinit ice depth=1; zinit light romkatv/powerlevel10k

# Add in zsh plugins.
zinit light zsh-users/zsh-syntax-highlighting
zinit light zsh-users/zsh-completions
zinit light zsh-users/zsh-autosuggestions
zinit light Aloxaf/fzf-tab

# Add in snippets.
# zinit snippet OMZP::git
# zinit snippet OMZP::sudo
# zinit snippet OMZP::archlinux
# zinit snippet OMZP::aws
# zinit snippet OMZP::kubectl
# zinit snippet OMZP::kubectx
# zinit snippet OMZP::command-not-found

# =============================================================================
# Completions.
# =============================================================================

# Load & run completions.
autoload -Uz compinit && compinit

# Make sure hidden folders and files can be completed (ones that start with .).
_comp_options+=(globdots)

# Replay completion definitions (to be done after compinit).
zinit cdreplay -q

# =============================================================================
# Misc options.
# =============================================================================

# No beeping please.
setopt NO_BEEP

# =============================================================================
# Powerlevel10k (powerline).
# =============================================================================

# To customize prompt, run `p10k configure` or edit ~/.p10k.zsh.
[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh

# =============================================================================
# History.
# =============================================================================

HISTSIZE=5000
HISTFILE=~/.zsh_history
SAVEHIST=$HISTSIZE
HISTDUP=erase
setopt appendhistory
setopt sharehistory
setopt hist_ignore_space
setopt hist_ignore_all_dups
setopt hist_save_no_dups
setopt hist_ignore_dups
setopt hist_find_no_dups

# =============================================================================
# Completion styling.
# =============================================================================

zstyle ':completion:*' matcher-list 'm:{a-z}={A-Za-z}'
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' menu no
zstyle ':fzf-tab:complete:cd:*' fzf-preview 'ls --color $realpath'
zstyle ':fzf-tab:complete:__zoxide_z:*' fzf-preview 'ls --color $realpath'

# =============================================================================
# Aliases.
# =============================================================================

# Allows easily attaching to existing tmux session.
alias tt="(tmux ls | grep -vx attached && tmux at) || tmux"
alias cat="bat"
alias ls="eza -la --icons --git"
alias lst="eza -la --icons --git --tree --level 2"

# =============================================================================
# Source other zsh files.
# =============================================================================

[[ ! -f ~/.vcs.zsh ]] || source ~/.vcs.zsh

# =============================================================================
# nvm (nodejs version manager).
# =============================================================================

export NVM_DIR="$HOME/.nvm"
# This loads nvm
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
# This loads nvm bash_completion
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# =============================================================================
# pnpm (nodejs package manager).
# =============================================================================

# pnpm
export PNPM_HOME="/Users/kad/Library/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME:"*) ;;
  *) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm endexport

# =============================================================================
# Shell integrations.
# =============================================================================

eval "$(fzf --zsh)"

# Note: This will replace the "cd" command with zoxide.
eval "$(zoxide init --cmd cd zsh)"
