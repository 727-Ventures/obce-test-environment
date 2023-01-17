{
  inputs = {
    nixpkgs = {
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
      ref = "nixos-unstable";
    };

    fenix = {
      type = "github";
      owner = "nix-community";
      repo = "fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      type = "github";
      owner = "ipetkov";
      repo = "crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-utils = {
      type = "github";
      owner = "numtide";
      repo = "flake-utils";
    };
  };

  outputs = {
    nixpkgs,
    fenix,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        fenixPkgs = fenix.packages.${system};

        rustToolchain = fenixPkgs.combine [
          (fenixPkgs.complete.withComponents [
            "rustc"
            "cargo"
            "clippy"
            "rustfmt"
            "rust-src"
          ])
          fenixPkgs.targets."wasm32-unknown-unknown".latest.rust-std
        ];

        craneLib =
          crane.lib.${system}.overrideToolchain
          rustToolchain;

        contractsNode = craneLib.buildPackage {
          src = ./.;

          cargoArtifacts = null;

          buildInputs = [
            pkgs.protobuf
          ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.protobuf
          ];
        };

        packages = {
          dockerIntegrationTest = pkgs.dockerTools.buildImage {
            name = "obce-docker-image";
            config = {
              Cmd = ["${contractsNode}/bin/substrate-contracts-node" "--dev" "--tmp"];
            };
          };
        };

        formatter = pkgs.alejandra;
      }
    );
}
