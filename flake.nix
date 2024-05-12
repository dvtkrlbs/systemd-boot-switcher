{
  description = "<ADD YOUR DESCRIPTION>";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crate2nix,
  }:
    flake-utils.lib.eachDefaultSystem (system: 
    let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      cargoNix = crate2nix.tools.${system}.appliedCargoNix {
        name = "crate-name";
        src = ./.;
      };
    in {
      devShells = {
        default = pkgs.mkShell {
          #inherit (self.checks.${system}.pre-commit-check) shellHook;
          buildInputs = with pkgs;
            [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            ]
            ++ lib.optionals stdenv.isDarwin [
              libiconv
              darwin.apple_sdk.frameworks.Security
            ];
        };
      };

      checks = {
        rustnix = cargoNix.rootCrate.build.override {
          runTests = true;
        };
      };

      packages = {
        default = cargoNix.rootCrate.build;

        inherit (pkgs) rust-toolchain;
      };

      # packages = { default = ... };
    });
}
