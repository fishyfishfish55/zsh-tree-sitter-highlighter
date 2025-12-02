# Tree-sitter custom highlighter for zsh-syntax-highlighting
# Path to your Rust parser binary
_zsh_treesitter_cmd="$HOME/.local/bin/zsh_tree_sitter_highlighter"

# This function tells zsh-syntax-highlighting whether to run our highlighter.
# For now, always return 0 (enabled for all lines).
_zsh_highlight_highlighter_treesitter_predicate() {
  return 0
}

# The actual highlighting function
_zsh_highlight_highlighter_treesitter_paint() {
  local buffer=$BUFFER

  # Reset highlights this pass
  region_highlight=()

  # Feed BUFFER to parser, get TSV output
  local start
  local end
  local kind
  while IFS=$'\t' read -r start end kind; do
    case $kind in
      command_name)
        region_highlight+=("$start $end fg=blue,bold")
        ;;
      error)
        region_highlight+=("$start $end fg=red,bold")
        ;;
      string|raw_string)
        region_highlight+=("$start $end fg=green")
        ;;
      variable_name)
        region_highlight+=("$start $end fg=yellow")
        ;;
      variable_assignment)
        region_highlight+=("$start $end fg=yellow,bold")
        ;;
      environment_variable_assignment)
        region_highlight+=("$start $end fg=orange,bold")
        ;;
      redirection|file_descriptor)
        region_highlight+=("$start $end fg=cyan,bold")
        ;;
      pipe)
        region_highlight+=("$start $end fg=brightcyan")
        ;;
      comment)
        region_highlight+=("$start $end fg=brightblack")
        ;;
      word)
        region_highlight+=("$start $end fg=default")
        ;;
      option|flag) # TODO: Implement
        region_highlight+=("$start $end fg=magenta")
        ;;
      *)
        region_highlight+=("$start $end fg=default")
        ;;
    esac
  done < <(echo "$buffer" | "$_zsh_treesitter_cmd")
}

