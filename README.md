# 🔍 minigrep (Upgraded Rust CLI Tool)

A powerful, extensible command-line search tool written in Rust. Inspired by `grep`, this upgraded version supports:

- ✅ Regex search
- 📂 Recursive directory scanning
- 🔠 Case-insensitive matching
- 📊 Summary reporting
- 🧪 Unit-tested functionality

---

## 🚀 Features

- **Regex Support** – Match complex patterns with the [`regex`](https://docs.rs/regex) crate
- **Recursive Search** – Automatically finds and searches files in subdirectories
- **Flexible CLI** – Built with [`clap`](https://docs.rs/clap) for easy argument parsing and `--help` support
- **Case-Insensitive Option** – Use `--ignore-case` for flexible matching
- **Summary Mode** – Use `--summary` to print total matches only
- **Line Numbers + File Paths** – Output shows where the match occurs

---

## 📦 Installation

```bash
git clone https://github.com/yourusername/minigrep-upgraded.git
cd minigrep-upgraded
cargo build --release
