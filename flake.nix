{
  description = "gitzeug: A CLI toolkit for streamlined Git repository interactions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        gitzeug = pkgs.rustPlatform.buildRustPackage rec {
          pname = "gitzeug";
          version = "0.4.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = [ pkgs.git ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          meta = {
            description = "A CLI toolkit for streamlined Git repository interactions";
            homepage = "https://github.com/thaemisch/gitzeug";
            license = pkgs.lib.licenses.mit;
            platforms = pkgs.lib.platforms.unix;
          };
        };
      in
      {
        packages = {
          default = gitzeug;
          gitzeug = gitzeug;
        };
        apps = {
          gitzeug = {
            type = "app";
            program = "${gitzeug}/bin/gitzeug";
          };
          default = self.apps.${system}.gitzeug;
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            git
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
