{
  description = "Rust language development flake";
  # Template from Vimjoyer-Effortless Rust Development with Nix Devshells, Packaging & Beyond
  # https://www.youtube.com/watch?v=Ss1IXtYnpsg

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = with pkgs; [
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
        # Expose dependencies declared above
        nativeBuildInputs = [ pkgs.pkg-config ];
        # Tools need path
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
}
