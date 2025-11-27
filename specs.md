# rs_tiktokenizer

This is a small side project to build a small CLI tool I would like to have, whilst learning the basics of Rust.
The goal of this cli is to to calculate the number of token a specific file (or set of files, or directory), for a specific tokenizer.

## Functionalities

### Example usage

In the example below, the cli can be invoked with the rst command.

#### Basic example: one file, with default tokenizer

```bash
rst example_file.md
```

#### with multiple files

```bash
rst -f example_file_1.md, example_file_2.py
```

#### with multiple files, for a specific model

```bash
rst -f example_file_1.md, example_file_2.py -t claude_sonnet_4-5
```

### Potential extentions

We might consider adding a vizual output in the terminal to display the tokens (like in the https://tiktokenizer.vercel.app app).

