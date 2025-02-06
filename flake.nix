{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    pkgsForAllSystems = f:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ] (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };
        });
  in {
    devShells = pkgsForAllSystems (
      {pkgs}: {
        default =
          pkgs.mkShell.override {
            stdenv = pkgs.clangStdenv;
          } {
            packages = with pkgs; [
              # rustToolchain
              # cargo-outdated
              # cargo-udeps
              # cargo-watch
              # curl
              # git
              # jq
              # wget
              # nixpkgs-fmt
              # rust
              taplo-lsp
              cargo-edit
              # cargo-about
              # cargo-readme
              # cargo-features
              # cargo-sort
              # cargo-realease
              # cargo-modules
              # cargo-deps
              # cargo-diet
              # cargo-bloat
              perl
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              openssl
              pkg-config
              gnumake
              gcc
            ];

            shellHook = ''
              export OPENSSL_DIR=${pkgs.openssl.dev}
              export PKG_CONFIG_PATH=${pkgs.openssl.bin}/pkgconfig
            '';
          };
      }
    );
  };
}
