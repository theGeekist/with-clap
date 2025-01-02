# `with-clap`

A simple template for building CLI applications with Rust. It includes `clap` for command-line argument parsing and structured logging with `pretty_env_logger`.

Perfect for developers who want a quick base for CLI tools. Generate projects effortlessly with a single command and let GitHub Actions handle dependency updates and checks for you!

---

## **Features**
- **🚀 Modern CLI Argument Parsing**: Powered by `clap` for intuitive and flexible command-line interfaces.
- **📜 Structured Logging**: Uses `pretty_env_logger` for clean and readable output.
- **📦 Serialization Support**: Demonstrates `serde` and `serde_json` for handling structured data.
- **🔄 Automated Maintenance**: Dependencies are updated and tested daily via GitHub Actions.
- **📄 Project Metadata**: Automatically incorporates `name`, `version`, and `authors` from `Cargo.toml`.

---

## **Getting Started**

### **Prerequisites**
Ensure you have Rust and Cargo installed on your system. If not, install them using [Rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### **Installation**
To use this template, you’ll need the `cargo-generate` tool installed globally:
```bash
cargo install cargo-generate
```

---

### **Generate a New Project**
To create a new project based on this template:
```bash
cargo generate --git https://github.com/theGeekist/with-clap.git --name my_project
```

---

### **Optional: Add a Handy Alias**
For convenience, create an alias in your shell configuration file (e.g., .bashrc, .zshrc):

bash
Copy code
```bash
alias rustnew='cargo generate --git https://github.com/theGeekist/with-clap.git --name'
source ~/.zshrc  # or ~/.bashrc
```
Now, you can quickly generate projects with:
```bash
rustnew my_project
```

---

## **Usage**
Once your project is generated, you can build and run it:

1. **Run the example CLI:**
   ```bash
   cargo run -- --user <YOUR_USERNAME>

   ```

2. **Run Tests:** Execute included tests to ensure functionality:
   ```bash
   cargo test
   ```

3. **Lint the Code:** Use Clippy to maintain code quality:
   ```bash
   cargo clippy -- -D warnings
   ```   

4. **Example Output:**
   ```bash
   Starting the application...
   Name provided: <PROJECT_NAME>
   Serialized Config: {"name":"<PROJECT_NAME>","version":"0.1.0"}
   Deserialized Config: Config { name: "<PROJECT_NAME>", version: "0.1.0" }
   ```

---

## **Why Use This Template?**

- ⏳ Eliminates boilerplate, letting you focus on building features.
- 🛠️ Incorporates modern Rust idioms and tools.
- 🔧 Easily customise it to suit your project needs.
- ⚙️ Dependencies are checked, updated, and tested daily via GitHub Actions.

---

## **Contributing**
Contributions are welcome! Feel free to:
- Submit issues or pull requests on the [GitHub repository](https://github.com/theGeekist/with-clap).
- Star the repository if you find this template helpful!


---

## **License**
This project is licensed under the [MIT License](LICENSE).


So I think that time signature should be optional and we should default to the usual suspects for ease of use, for example, 4/4, 3/4, 6/8 etc. Because many people don't care for this much and it would be nice to guide them along

Also, I think that it would be prudent to always display the computed bpm for all the given time signatures - unless one was chosen via the interactive prompt or flag.

Also I have a new base rust project template that I am now using:
