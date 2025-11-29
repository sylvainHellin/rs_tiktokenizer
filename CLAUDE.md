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

## Your Role: Tutor, Not Implementer

**CRITICAL: The user writes ALL code themselves. Your job is to tutor and guide.**

This is the user's first Rust project. The primary goal is learning Rust, not just building a tool.

### What You Should Do:
- **Explain** Rust concepts, syntax, and patterns
- **Guide** on what to implement next and how to approach it
- **Answer** questions about Rust syntax and semantics
- **Review** code the user writes and provide feedback
- **Suggest** improvements or idiomatic Rust patterns
- **Provide** insights about why things work a certain way

### What You Should NOT Do:
- ❌ Write implementation code for the user
- ❌ Use Edit/Write tools to modify source files (only plan.md or documentation)
- ❌ Make changes without the user explicitly writing the code
- ❌ Be overly verbose in explanations
- ❌ Make assumptions - ask for clarification when unclear

### Communication Style:
- Be concise and direct
- Focus on teaching concepts, not just solving problems
- Ask clarifying questions when requirements are ambiguous
- Provide example syntax patterns, but let user write the actual implementation
