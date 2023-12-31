{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs @ {
    flake-parts,
    fenix,
    crane,
    treefmt-nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        treefmt-nix.flakeModule
      ];
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: let
        rust-toolchain = fenix.packages.${system}.fromToolchainFile {
          dir = ./.;
          sha256 = "sha256-R0F0Risbr74xg9mEYydyebx/z0Wu6HI0/KWwrV30vZo=";
        };
        crane-lib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
        src = crane-lib.cleanCargoSource (crane-lib.path ./.);
        commonArgs = {
          inherit src;
          pname = "rodos";
          version = "0.1.0";
          buildInputs =
            [
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
            ];
          nativeBuildInputs = [
            rust-toolchain
          ];
        };
        cargoArtifacts = crane-lib.buildDepsOnly commonArgs;
        bin = crane-lib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
          });
      in {
        checks = {
          inherit bin;
          clippy = crane-lib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });
        };
        packages = {
          default = bin;
        };
        devShells.default = pkgs.mkShell {
          name = "devshell";
          inherit (commonArgs) nativeBuildInputs buildInputs;
          packages = with pkgs; [
            config.treefmt.build.wrapper
            tokei
          ];
        };

        treefmt.config = {
          projectRootFile = "flake.nix";
          programs.rustfmt.enable = true;
          programs.alejandra.enable = true;
          settings.formatter.alejandra.options = ["-q"];
        };
      };
      flake = {
      };
    };
}
