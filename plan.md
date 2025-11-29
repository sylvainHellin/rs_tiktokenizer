# Implementation Plan for rs_tiktokenizer

## Project Goal
Build a CLI tool (`rst`) that calculates token counts for files using various model tokenizers.

## Progress Tracker

### ✅ Completed
1. **CLI Argument Parsing (Basic)**
   - Added `clap` dependency with derive features
   - Created `Args` struct with:
     - `files: Vec<String>` - accepts multiple file paths
     - `tokenizer: String` - flag `-t/--tokenizer` with default "cl100k_base"
   - Implemented basic parsing in `main()`
   - Tested compilation and basic arg parsing

## Implementation Roadmap

### Phase 1: File Processing ⏳ Next
**Goal:** Read and validate file inputs

**Tasks:**
1. Handle single file input
2. Handle multiple files
3. Handle directory input (recursively read files)
4. Error handling:
   - Non-existent files (skip with error message)
   - Binary files (skip with warning)
   - Empty files (report 0 tokens)
5. Filter for text files only

**Key Rust Concepts:**
- `std::fs` for file operations
- `std::path::Path` and `PathBuf` for path handling
- `Result<T, E>` for error handling
- Pattern matching with `match` or `if let`

### Phase 2: Tokenizer Integration
**Goal:** Integrate a tokenizer library

**Tasks:**
1. Research Rust tokenizer libraries:
   - `tiktoken-rs` for OpenAI tokenizers (cl100k_base, o200k_base)
   - Need to find library for Claude tokenizers
2. Add tokenizer dependency to `Cargo.toml`
3. Create tokenizer initialization logic based on `--tokenizer` flag
4. Handle unknown tokenizer names

**Key Rust Concepts:**
- External crate usage
- Enums for tokenizer types
- Error propagation with `?` operator

### Phase 3: Token Counting
**Goal:** Count tokens for each file

**Tasks:**
1. Read file contents as strings
2. Pass content through selected tokenizer
3. Store results (filename + token count)
4. Accumulate total count

**Key Rust Concepts:**
- Collections: `Vec`, `HashMap`
- String vs &str
- Iterators and `map()`/`fold()`

### Phase 4: Output Formatting
**Goal:** Display results matching spec format

**Tasks:**
1. Print individual file counts: `file.md: 1,234 tokens`
2. Add separator: `---`
3. Print total: `Total: 1,801 tokens`
4. Format numbers with commas

**Key Rust Concepts:**
- String formatting with `format!` macro
- Number formatting

### Phase 5: CLI Refinement
**Goal:** Add remaining CLI features

**Tasks:**
1. Add `--help` message with usage examples
2. Add `--version` flag
3. Validate tokenizer names
4. Add helpful error messages

**Key Rust Concepts:**
- Clap's built-in help generation
- Custom validation

## Future Enhancements (Post-MVP)
- Visual token display in terminal
- Export to JSON/CSV
- Watch mode for live counting
- Performance optimization for large files

## Notes
- User writes all code (learning exercise)
- Focus on understanding Rust concepts
- Ask questions when syntax is unclear
