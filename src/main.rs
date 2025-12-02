use std::collections::HashMap;
use std::io::{self, Read};
use tree_sitter::{Parser, Node};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct Token {
    start: usize,
    end: usize,
    kind: String,
}

/// Assign a priority to each node kind
fn kind_priority(kind: &str) -> u8 {
    match kind {
        "error" => 100,
        "comment" => 3,
        "string" | "raw_string" => 3,
        "command_name" => 3,
        "variable_assignment" => 3,
        "word" => 2,
        // generic / structural nodes: lowest priority
        "command" | "program" => 1,
        _ => 2, // default medium
    }
}

fn collect_tokens(node: Node, tokens: &mut Vec<Token>) {
    if node.is_error() {
        tokens.push(Token {
            start: node.start_byte(),
            end: node.end_byte(),
            kind: "error".to_string(),
        });
    } else if node.is_named() {
        tokens.push(Token {
            start: node.start_byte(),
            end: node.end_byte(),
            kind: node.kind().to_string(),
        });
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_tokens(child, tokens);
        }
    }
}

/// Deduplicate tokens by priority + overlap
fn deduplicate_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
    // Sort: higher priority first, then shorter spans
    tokens.sort_by(|a, b| {
        kind_priority(&b.kind)
            .cmp(&kind_priority(&a.kind))
            .then((a.end - a.start).cmp(&(b.end - b.start)))
    });

    let mut kept: Vec<Token> = Vec::new();

    'outer: for tok in tokens {
        // If contained by an existing kept token → skip
        for existing in &kept {
            if tok.start >= existing.start && tok.end <= existing.end {
                continue 'outer;
            }
        }

        // If it *contains* a kept token of >= priority → skip
        for existing in &kept {
            if tok.start <= existing.start && tok.end >= existing.end {
                if kind_priority(&tok.kind) <= kind_priority(&existing.kind) {
                    continue 'outer;
                }
            }
        }

        kept.push(tok);
    }

    kept
}

/// Merge adjacent word tokens into one, allowing only whitespace in between
fn merge_adjacent_words(mut tokens: Vec<Token>, input: &str) -> Vec<Token> {
    tokens.sort_by_key(|t| t.start);

    let mut merged: Vec<Token> = Vec::new();

    for tok in tokens {
        if let Some(last) = merged.last_mut() {
            if last.kind == "word" && tok.kind == "word" {
                // Extract the substring between tokens
                let between = &input[last.end..tok.start];
                if between.chars().all(|c| c.is_whitespace()) {
                    // Merge into the last token
                    last.end = tok.end;
                    continue;
                }
            }
        }
        merged.push(tok);
    }

    merged
}

fn main() {
    // Read the entire input line from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    // Init parser
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_bash::language())
        .expect("Error loading bash grammar");

    let tree = match parser.parse(&input, None) {
        Some(t) => t,
        None => return,
    };

    let mut tokens = Vec::new();
    collect_tokens(tree.root_node(), &mut tokens);

    let tokens = deduplicate_tokens(tokens);
    let tokens = merge_adjacent_words(tokens, &input);

    for tok in tokens {
        println!("{}\t{}\t{}", tok.start, tok.end, tok.kind);
    }
}

