{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    rust-overlay.url = "github:oxalica/rust-overlay";
    # crane.url = "github:ipetkov/crane";
    # crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
            ];
          };
          rustBuildInputs = [
            # pkgs.libiconv
          ] ++ lib.optionals pkgs.stdenv.isLinux [
            # pkgs.glib
            pkgs.clang
            pkgs.libllvm
          ];

          # This is useful when building crates as packages
          # Note that it does require a `Cargo.lock` which this repo does not have
          # craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };

          devShells.default = pkgs.mkShell {
            name = "dioxus-dev";
            buildInputs = rustBuildInputs;
            nativeBuildInputs = [
              # Add shell dependencies here
              rustToolchain
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
              export PS1='\n\[\033[1;34m\](Rust):\w]\$\[\033[0m\]';
              export LIBCLANG_PATH = "${pkgs.libllvm.lib}/lib";
            '';
          };
        };
    };
}