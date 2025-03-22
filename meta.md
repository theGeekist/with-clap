# META: WITH-CLAP

## Folder Structure

```plaintext
/Users/jasonnathan/Repos/with-clap
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    └── release

5 directories, 6 files
```
## File: README.md

```md
# `with-clap`

A simple template for building CLI applications with Rust. It includes `clap` for command-line argument parsing and structured logging with `pretty_env_logger`.

Perfect for those who want a quick base for CLI tools. Generate projects effortlessly with a single command and let GitHub Actions handle dependency updates and checks for you!

---

## Features
- **🚀 Modern CLI Argument Parsing**: Powered by `clap` for intuitive and flexible command-line interfaces.
- **📜 Structured Logging**: Uses `pretty_env_logger` for clean and readable output.
- **📦 Serialization Support**: Demonstrates `serde` and `serde_json` for handling structured data.
- **🔄 Automated Maintenance**: Dependencies are updated and tested daily via GitHub Actions.

---

## Getting Started

### Prerequisites
Ensure you have Rust and Cargo installed on your system. If not, install them using [Rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### Installation
To use this template, you’ll need the `cargo-generate` tool installed globally:
```bash
cargo install cargo-generate
```

---

### Generate a New Project
To create a new project based on this template:
```bash
cargo generate --git https://github.com/theGeekist/with-clap.git --name my_project
```

---

### Optional: Add a Handy Alias
For convenience, create an alias in your shell configuration file (e.g., `.bashrc`, `.zshrc`):
```bash
alias rustnew='cargo generate --git https://github.com/theGeekist/with-clap.git --name'
source ~/.zshrc  # or ~/.bashrc
```
Now, you can quickly generate projects with:
```bash
rustnew my_project
```

---

## Usage
Once your project is generated, you can build and run it:

1. **Run the example CLI**:
   ```bash
   cargo run -- --user <YOUR_USERNAME>
   ```
   **Expected Output**:
   ```bash
   [INFO  with-clap] Starting the application...
   [INFO  with-clap] Parsing command-line arguments...
   [WARN  with-clap] This is a sample warning log for testing purposes.
   Serialized Config: {"user":"<YOUR_USERNAME>","version":"0.1.3"}
   Deserialized Config: Config { user: "<YOUR_USERNAME>", version: "0.1.3" }
   ```
   > ⚠️ **Note**: The included example application is a simple demonstration of the template’s capabilities, such as argument parsing, structured logging, and JSON serialization. Feel free to replace or extend it to fit your project needs.

2. **Run Tests**:
   Execute included tests to ensure functionality:
   ```bash
   cargo test
   ```

3. **Lint the Code**:
   Use Clippy to maintain code quality:
   ```bash
   cargo clippy -- -D warnings
   ```

---

## Why Use This Template?

- ⏳ Eliminates boilerplate, letting you focus on building features.
- 🛠️ Incorporates modern Rust idioms and tools.
- 🔧 Easily customise it to suit your project needs.
- ⚙️ Dependencies are checked, updated, and tested daily via GitHub Actions.

---

## Contributing
Contributions are welcome! Feel free to:
- Submit issues or pull requests on the [GitHub repository](https://github.com/theGeekist/with-clap).
- Star the repository if you find this template helpful!

---

## License
This project is licensed under the [MIT License](LICENSE).
```

## Git Repository

```plaintext
origin	git@github.com:theGeekist/with-clap.git (fetch)
origin	git@github.com:theGeekist/with-clap.git (push)
```
