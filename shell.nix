{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.cargo
    pkgs.rustc
  ];

  # This makes pkg-config see openssl
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}