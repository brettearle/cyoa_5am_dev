# Choose Your Own Adventure – Rust CLI Game

Welcome to the **Choose Your Own Adventure (CYOA)** project — a simple, terminal-based interactive fiction game built with Rust.

This repository is part of the [5 AM Dev YouTube series](https://www.youtube.com/playlist?list=PLt82tLcobUvrt6tbP8m_uLGDnlSxMjDjE), where we build a complete CLI game step by step using only Rust's standard library.

## 📚 Requirements

All gameplay and development requirements are documented in [`REQUIREMENTS.md`](./REQUIREMENTS.md).

The game's content is defined externally in [`story.json`](./story.json), allowing you to expand or change the story without touching any Rust code.

## ▶️ Getting Started

This project was created with `cargo new` and requires only the Rust toolchain.

### Build
```bash
cargo build
````

### Run

```bash
cargo run
```

## 🧠 Learning Outcomes

By following this series, you'll learn how to:

1. **Understand control flow** using conditionals, loops, and input validation.
2. **Work with structured data** (JSON) for dynamic content loading.
3. **Parse and traverse nested data structures** like maps and arrays.
4. **Build a text-based CLI interface** using standard input/output.
5. **Separate content from logic** with external story files.
6. **Handle user input safely**, including feedback for invalid choices.
7. **Implement a finite state navigation system** (moving between story nodes).
8. **Write portable code** without relying on external crates.
9. **Structure code for clarity and maintainability**, using modularity and comments.
10. **(Optional)** Add helpful CLI flags such as `--help`.

## 📺 Watch the Series

Follow along and code with me:
[👉 YouTube Playlist: Choose Your Own Adventure in Rust](https://www.youtube.com/playlist?list=PLt82tLcobUvrt6tbP8m_uLGDnlSxMjDjE)

## 🔗 GitHub

Project source and updates live here:
[👉 GitHub Repo](https://github.com/brettearle/cyoa_5am_dev)

