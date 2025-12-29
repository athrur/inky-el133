{
  description = "Barebones driver for the 13.3\" Inky Impression e-ink display (EL133UF1)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, treefmt-nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Rust with ARM target included
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "armv7-unknown-linux-gnueabihf" ];
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs.nixpkgs-fmt.enable = true;
          programs.rustfmt.enable = true;
        };
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Compilation
            rustToolchain
            pkgs.zig
            pkgs.cargo-zigbuild

            # Deployment tools
            pkgs.just
            pkgs.openssh
          ];

          shellHook = ''
            echo "🦀 Inky Impression Rust Development Environment"
            echo ""
            echo "Run 'just' to see available commands"
            echo ""
          '';

          # Set environment variables
          RUST_BACKTRACE = "1";
        };
      }
    );
}
