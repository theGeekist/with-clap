# {{project-name}}

A template CLI project built using Rust and `clap`.

## Features
- Automatically includes project metadata (name, version, author) from `Cargo.toml`.
- Provides a basic structure for command-line argument parsing.

## Installation

- **Ensure you have `cargo-generate` installed**:
```bash
cargo install cargo-generate
```
- **Command to Generate a New Project**:
```bash
cargo generate --git https://github.com/theGeekist/with-clap.git --name my_project
```

- **Alias for Convenience**:
```bash
alias rustnew='cargo generate --git https://github.com/theGeekist/with-clap.git --name'
```

## Usage
```bash
# Run the binary
cargo run -- --name <PROJECT_NAME>
