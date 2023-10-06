{
  description = "Liam's Rust template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Compile-time dependencies
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
          nodePackages.pnpm
          pscale
          mysql80
        ];

        # Runtime dependencies
        buildInputs = with pkgs; [ ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          inherit nativeBuildInputs buildInputs;
        };
      }
    );
}
