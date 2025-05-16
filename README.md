# gitzeug

A CLI toolkit for streamlined Git operations.

## About
Git is a very powerfull tool. But sometimes, I don't need powerfull. I need simple, easy, fast.

**gitzeug** is a command-line utility designed to simplify Git tasks. 

The name **gitzeug** is inspired by the German word *Werkzeug*, which means "tool." The etymology of *Werkzeug* comes from *werk* ("work") and *zeug* ("stuff"). Sooo ... a tool for git, basically. Thanks for the inspiration, [Werkzeug](https://github.com/pallets/werkzeug).

## Features

- **Download Specific Files or Directories**: Clone only the files or directories you need from a Git repository using sparse checkout.
- **Streamlined Push Workflow**: Stage all changes, commit with a message, and push in a single command.
- **Squash**: Squash the last n commits with a custom commit message.
- **Browse**: Browse the file tree of the repo without cloning it.

## Requirements
✅ `git`
## Installation

### Nixos
1. Add this GitHub repository to the inputs of your flake.nix:
   ```nix
   gitzeug.url = "github:thaemisch/gitzeug";
   ```
2. Add the package to your configuration.nix:
   ```nix
   environment.systemPackages = with pkgs; [
      ...
      inputs.gitzeug.packages.${pkgs.system}.gitzeug
   ];
   ```

### Windows, Arch Linux, Debian & Others
Adding the package to apt, aur and winget is planned at a more mature state of the project.

### Build from source
1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:
   ```sh
   git clone https://github.com/thaemisch/gitzeug.git
   ```
3. Navigate to the project directory:
   ```sh
   cd gitzeug
   ```
4. Build the project:
   ```sh
   cargo build --release
   ```
5. Add the binary to your PATH for easy access:
   ```sh
   export PATH=$PATH:$(pwd)/target/release
   ```

## Usage
```sh 
gitzeug -h
```
```sh 
gitzeug <command> -h
```
## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.
