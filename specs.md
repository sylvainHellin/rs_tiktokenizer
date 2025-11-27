# rs_tiktokenizer

CLI tool to calculate token counts for files using various model tokenizers.

## Usage

```bash
# Single file (uses cl100k_base tokenizer by default)
rst example.md

# Multiple files
rst file1.md file2.py file3.txt

# Entire directory
rst src/

# Specify tokenizer
rst -t o200k_base file.md
rst -t claude-sonnet-4-5 src/
```

## Output Format

```
file.md: 1,234 tokens
file.py: 567 tokens
---
Total: 1,801 tokens
```

## Supported Tokenizers

- `cl100k_base` (default) - GPT-4, GPT-3.5-turbo
- `o200k_base` - GPT-4o
- `claude-sonnet-4-5` - Claude Sonnet 4.5
- `claude-opus-4` - Claude Opus 4

## Options

- `-t, --tokenizer <NAME>` - Specify tokenizer (default: cl100k_base)
- `-h, --help` - Show help message
- `-v, --version` - Show version

## Error Handling

- Invalid file paths: show error, skip file, continue with others
- Empty files: report 0 tokens
- Binary files: skip with warning

## Future Extensions

- Visual token display in terminal (like tiktokenizer.vercel.app)
- Export results to JSON/CSV
- Watch mode for live token counting

