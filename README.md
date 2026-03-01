# 🦀 Cargo Purge

**Cargo Purge** is a fast CLI tool for bulk cleaning Rust projects. It recursively finds all directories containing a `Cargo.toml` file and runs `cargo clean` on each of them.

The key feature is **smart traversal**: the program skips heavy directories like (`target`, `.git`, `node_modules`) and stops searching deeper into a branch once a Rust project root is found.

---

## 🚀 Installation

### From Source
Ensure you have Rust installed. Clone the repository or navigate to the project folder and run:

```bash
cargo install --path .
```

This command compiles the binary in `release` mode and copies it to `~/.cargo/bin`. The tool will then be available globally as `cargo-purge`.

---

## 🛠 Usage

The tool works in two stages: **find** and **clear**.

### 1. Find Projects (`find`)
By default, it searches in the current working directory:
```bash
cargo-purge find
```

You can specify a custom path and add directories to the exclusion list:
```bash
cargo-purge find ~/projects --exclude my_backup --exclude old_versions
```

**How it works:** The program creates a cache file named `.found_projects.txt` containing the discovered paths. You can review this list before proceeding to clean.



### 2. Clean Projects (`clear`)
Executes `cargo clean` in all previously discovered projects:
```bash
cargo-purge clear
```

**Safety:** The tool visits each project directory sequentially and invokes the official `cargo clean` command. Once finished, the cache file is automatically deleted.

---

## 📂 Ignored Directories
For maximum performance, the program automatically skips:
* `target`, `.git`, `node_modules`, `.idea`, `.vscode`, `build`, `venv`.
* Any subdirectories inside an already identified Rust project (tree pruning).



---

## ⚙️ Technical Details

* **Cache File:** `.found_projects.txt` is created in the directory where `find` was executed. You must run the `clear` command from the same directory.
* **Requirements:** Rust 1.56+ and `cargo` available in your system PATH.

---

## 📝 License
MIT. Enjoy!
