# `with-clap`

A simple template for building CLI applications with Rust, it includes `clap` for command-line argument parsing and structured logging with `pretty_env_logger`. 

Perfect for folks who want a quick base for CLI tools so you can simply use `rustnew my_project` when you create new rust projects.

Dependancies are checked, updated & tested every day via Github Actions!

---

## **Features**
- **Project Metadata**: Automatically incorporates `name`, `version`, and `author` from `Cargo.toml`.
- **CLI Argument Parsing**: Leverages `clap` for intuitive argument handling.
- **Structured Logging**: Includes `pretty_env_logger` for clean, structured output.
- **Serialization Support**: Demonstrates `serde` and `serde_json` for working with structured data.

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
For convenience, create an alias in your shell configuration:
```bash
alias rustnew='cargo generate --git https://github.com/theGeekist/with-clap.git --name'
```
Now, you can quickly generate projects with:
```bash
rustnew my_project
```

---

## **Usage**
Once your project is generated, you can build and run it:

1. **Run the CLI:**
   ```bash
   cargo run -- --name <YOUR_NAME>
   ```

2. **Output Example:**
   ```bash
   Starting the application...
   Name provided: <YOUR_NAME>
   Serialized Config: {"name":"<YOUR_NAME>","version":"0.1.0"}
   Deserialized Config: Config { name: "<YOUR_NAME>", version: "0.1.0" }
   ```

---

## **Why Use This Template?**

- **Time-Saving**: Eliminates boilerplate, letting you focus on building features.
- **Best Practices**: Incorporates modern Rust idioms and tools.
- **Extensible**: Easily customise it to suit your project needs.
- **Automated Maintenance**: Dependencies are checked, updated, and tested daily via GitHub Actions.

---

## **Contributing**
Contributions are welcome! Feel free to:
- Submit issues or pull requests on the [GitHub repository](https://github.com/theGeekist/with-clap).
- Star the repository if you find this template helpful!


---

## **License**
This project is licensed under the [MIT License](LICENSE).
