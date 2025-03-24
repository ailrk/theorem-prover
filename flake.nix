{
  description = "A simple Rust project using Nix flakes and flake-utils without rust-overlay";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable"; # Adjust to your preferred version
    flake-utils.url = "github:numtide/flake-utils";         # Import flake-utils
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: {
    packages.default = nixpkgs.legacyPackages.${system}.callPackage ./default.nix { };

    devShell = nixpkgs.legacyPackages.${system}.mkShell {
      buildInputs = [
        nixpkgs.legacyPackages.${system}.rustc
        nixpkgs.legacyPackages.${system}.cargo
        nixpkgs.legacyPackages.${system}.rust-analyzer
      ];
    };
  });
}

