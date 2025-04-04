{
  description = "A devShell with rust-overlay";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            cargo-llvm-cov
            (rust-bin.beta.latest.default.override {
              extensions = [
                "llvm-tools-preview"
              ];
            })
          ];

          shellHooks = ''
            alias v=nvim
          '';
        };
      }
    );
}

