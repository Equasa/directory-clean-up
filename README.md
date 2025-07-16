# directory-clean-up

**Little application that you can run to cleanup a directory**

## Features

* **Interactive cleanup**: Lists files and directories in the current working directory and prompts for confirmation before deletion.
* **Lightweight**: No additional configuration files or dependencies beyond Rust and the `crossterm` crate.
* **Immediate quit**: Press `q` at any prompt to exit the application.

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) 
* [Git](https://git-scm.com/downloads)

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/Equasa/directory-clean-up.git
   cd directory-clean-up
   ```

2. **Build in release mode**

   ```bash
   cargo build --release
   ```

3. **(Optional) Install globally**

   ```bash
   cargo install --path .
   ```

## Usage

Run the tool from within the directory you want to clean:

```bash
# If installed globally
clean-dir

# Or directly via Cargo
cargo run --release
```

Example session:

```text
The current directory is: /home/user/projects/my-app/
Press q to quit

src/
Remove directory? [y/n] y

README.md
Remove file? [y/n] n

Directory cleaned
```

## Project Structure

```
├── Cargo.toml         # Project metadata and dependencies (crossterm)
├── .gitignore         
└── src
    ├── main.rs        # Entry point: lists and deletes objects
    └── utils.rs       # Helper functions: ls, confirm, delete_object
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is currently unlicensed. Contact the author for licensing inquiries.
