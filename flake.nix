{
  description = "A simple Rust project using Nix flakes and flake-utils without rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/8f3cf34b8d2e2caf4ae5ee1d1fddc1baab4c5964";
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

