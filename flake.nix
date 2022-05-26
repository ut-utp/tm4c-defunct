{
  description = "UTP TM4C Flake";

  inputs = {
    nixpkgs.url      = github:NixOS/nixpkgs/nixos-21.11;
    rust-overlay.url = github:oxalica/rust-overlay;
    flake-utils.url  = github:numtide/flake-utils;
  };

  # TODO: cargo extensions
  # TODO: CI setup? (garnix)
  # TODO: expose targets, etc.
  # TODO: flip-link, tests, etc.

  # TODO: openocd, gdb-dashboard, etc.

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ];
          shellHook = ''
          '';
        };
      }
    );
}
