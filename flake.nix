{
  description = "A reproducible Rust development environment with modern tooling.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
            pkgs.rust-analyzer
          ];
          shellHook = ''
            echo "Entering Rust development environment..."
            echo "Available tools: cargo, rustc, rustfmt, clippy, rust-analyzer"

            # Activate git hooks
            if git rev-parse --git-dir > /dev/null 2>&1; then
              git config core.hooksPath .githooks
              echo "Git hooks activated (.githooks/)"
            fi
          '';
        };
      }
    );
}
