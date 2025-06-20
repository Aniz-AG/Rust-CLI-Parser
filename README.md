# Rust CLI Text Analyzer 🔍📊

A simple but extensible command-line tool built in Rust that analyzes and searches through `.txt` files.

## 🚀 Features

- 🔎 Analyze a file: count characters, words, lines, average word length, longest word
- 🧠 Smart search: find all lines containing a specific word (case-insensitive)
- 📦 Output analysis in JSON format (optional)
- 📂 Modular structure (easy to extend with subcommands or new parsers)

## 🛠 Usage

### Analyze a file
```bash
cargo run -- analyze --file sample.txt
```
### **Output JSON**
```bash
cargo run -- analyze --file sample.txt --json
```
### **Output JSON**
```bash
Search for a word
cargo run -- search --file sample.txt --word Redis
```
✨ Built With
Rust
clap
serde
serde_json

💡 Future Ideas
Regex search

Word frequency histograms

Export to file

Interactive mode (TUI)
