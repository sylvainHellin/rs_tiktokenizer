# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rs_tiktokenizer` is a CLI tool for calculating token counts for files using various tokenizers. The project is also a learning exercise for Rust development.

The CLI will be invoked using the `rst` command and should support:
- Single file analysis: `rst example_file.md`
- Multiple files: `rst -f file1.md,file2.py`
- Specific tokenizers: `rst -f file1.md -t claude_sonnet_4-5`

## Build and Development Commands

```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the project
cargo run

# Run with arguments
cargo run -- <args>

# Run tests
cargo test

# Run a specific test
cargo test <test_name>

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy
```

## Project Architecture

This is an early-stage project with minimal structure:
- `src/main.rs` - Entry point (currently placeholder)
- `Cargo.toml` - Project manifest (no dependencies yet)
- `specs.md` - Feature specifications and usage examples

The project needs implementation of:
1. CLI argument parsing (likely using `clap` crate)
2. File/directory reading and processing
3. Tokenizer integration (needs research for appropriate Rust tokenizer library)
4. Token counting logic for different model tokenizers
5. Output formatting

Future consideration: Terminal visualization of tokens similar to https://tiktokenizer.vercel.app

## Answering guidelines

This is my first Rust project. One of the primary goals of this project is to improve my understanding of the language. Rather than implementing everything yourself, I would like you to help me implement the features with proper syntax, context and systemic support.
- Don't be too verbose.
- Don't make any assumptions. If something is unclear, please ask for clarification.
