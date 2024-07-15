# AstroCommit
AstroCommit is a command-line tool implemented in Rust that generates git commit messages based on astrological configurations. It uses astrological positions such as the Sun and Moon to inspire unique commit messages using AI-generated text.

# Features

1. **Astrological Configurations**: Calculates positions of the Sun and Moon based on provided date and time.
2. **AI-Powered Text Generation**: Generates commit messages using AI models (rust-bert).
3. **Command-Line Interface**: Simple CLI for easy interaction.

# Requirements

- Rust Installation (stable)
- Cargo (Rust's package manager)

# Installation

1. Clone the repository:
```bash
git clone https://github.com/Ab20zip/astro-commit.git
cd astro-commit
```

2. Install dependencies using Cargo:
```bash
cargo build
```

# Usage

Run AstroCommit with optional date and time arguments to generate commit messages:
```bash
cargo run -- --date 1993-06-21 --time 12:00
```

Replace `1993-06-21` and `12:00` with your desired date and time. If not provided, current date and time are used.

# License

This project, "AstroCommit" is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.

# Acknowledgements

- Rust Programming Language
- Clap—Command Line Argument Parser for Rust
- Chrono—Date and Time Handling in Rust
- rust-bert—Rust interface for BERT and other transformer models

<!-- MADE WITH ❤️ BY Ab20zip -->
