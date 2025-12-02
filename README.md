# ZSH Tree Sitter highlighter

This project is a [zsh-syntax-highlighting](https://github.com/zsh-users/zsh-syntax-highlighting/tree/master) highlighter that leverages [Tree Sitter](https://tree-sitter.github.io/tree-sitter/) for (hopefully) IDE-quality syntax highlighting.

# Installation
Make sure that you have the zsh-syntax-highlighting plugin installed.

Compile the project with 
```bash
cargo build --release
```
and copy `target/release/zsh_tree_sitter_highlighter` to a location in your PATH

Then add `treesittdr-highlighter.zsh` to zsh-syntax-highlighting's plugins folder (for help see [zsh-syntax-highlighting's documentation](https://github.com/zsh-users/zsh-syntax-highlighting/blob/master/docs/highlighters.md))

# To Do

Currently, the project uses [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) for rudimentary highlighting. Bash and ZSH aren't compatable for anything beyond rudementary use-cases. I'd like to make my own tree-sitter-zsh grammar.

Also, the highlighter only detects and highlights syntax errors at the moment. While I could implement rudementary logic in the ZSH script to check whether commands are valid, I'd rather create a ZSH language server (using [LSP](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)) that can detect errors in real-time
