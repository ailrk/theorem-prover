{
  description = "FOL theorem prover";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/8f3cf34b8d2e2caf4ae5ee1d1fddc1baab4c5964";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      system = system;
    };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "theorem-prover";
      version = "0.1.0";

      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };

    devShell = pkgs.callPackage ./shell.nix {};
  }
  );
}
