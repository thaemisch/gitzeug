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

### Download Files or Directories

```sh
gitzeug dl -u <repo-url> -o <output-dir> -f <files-or-directories>
```

- `-u` or `--url`: The URL of the Git repository.
- `-o` or `--output`: The output directory where files will be saved.
- `-f` or `--files`: A comma-separated list of files or directories to download.

Example:
```sh
gitzeug dl -u https://github.com/thaemisch/gitzeug.git -o ./output -f README.md,src
```

### Push Changes

```sh
gitzeug push <commit-message>
```

- `<commit-message>`: The message for the commit.

Example:
```sh
gitzeug push "Updated README"
```
### Squash
```sh 
gitzeug squash <number> <commit-message>
```

- `<number>`: Number of commits to Squash.
- `<commit-message>`: The message for the commit.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.
