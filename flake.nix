{
  description = "Example Rust development environment for Zero to Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            clippy
            cargo-edit
            cargo-binstall
            bacon
            openssl
            pkg-config
            # Add rust-analyzer separately to ensure it's the latest version
            # rust-analyzer
          ];

          shellHook = ''
            export PATH=${rustToolchain}/bin:$PATH
            export RUSTC_VERSION=$(rustc --version)
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
            # Add this line to explicitly set the rust-analyzer path
            export RUST_ANALYZER_PATH="${pkgs.rust-analyzer}/bin/rust-analyzer"
          '';

          packages = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv ]);
        };
      }
    );
}
