{
  description = "Rust language development flake";
  # Template from Vimjoyer-Effortless Rust Development with Nix Devshells, Packaging & Beyond
  # https://www.youtube.com/watch?v=Ss1IXtYnpsg
  # and NixOS templates
  # https://github.com/NixOS/templates/blob/master/rust/flake.nix

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              # Rust compiler, package and task management
              rustc
              cargo
              # Rust Linter
              clippy
              # Rust LSP
              rust-analyzer
              # Rust formatter
              rustfmt
              # Other dependencies
              # glib
            ];
            # Tools need path
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            # Expose dependencies declared above
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
      }
    );
}
