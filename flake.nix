{
  description = "A Nix-flake-based Rust development environment";

  nixConfig = {
    extra-substituters = [
      "https://fenix.cachix.org"
    ];
    extra-trusted-public-keys = [
      "fenix.cachix.org-1:ecJhr+RdYEdcVgUkjruiYhjbBloIEGov7bos90cZi0Q="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {self, ...} @ inputs: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];
    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (
        system: let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.self.overlays.default
            ];
          };
        in
          f {
            inherit system pkgs;
            preCommitCheck = inputs.git-hooks.lib.${system}.run {
              src = ./.;
              hooks = {
                alejandra.enable = true;
                clippy = {
                  enable = true;
                  packageOverrides = {
                    cargo = pkgs.rustToolchain;
                    clippy = pkgs.rustToolchain;
                  };
                  settings = {
                    allFeatures = true;
                    denyWarnings = true;
                  };
                };
                rustfmt = {
                  enable = true;
                  packageOverrides = {
                    cargo = pkgs.rustToolchain;
                    rustfmt = pkgs.rustToolchain;
                  };
                };
                check-toml.enable = true;
                taplo.enable = true;
              };
            };
          }
      );
  in {
    overlays.default = final: prev: {
      rustToolchain = inputs.fenix.packages.${prev.stdenv.hostPlatform.system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-A1abGIbOtcBSdrUMhDGrER3pRM1hQP4fp9gh3Y4PKc8=";
      };
    };

    devShells = forEachSupportedSystem (
      {
        pkgs,
        system,
        preCommitCheck,
        ...
      }: {
        default = pkgs.mkShell {
          shellHook = preCommitCheck.shellHook;
          packages = with pkgs;
            [
              rustToolchain
              openssl
              pkg-config
              rust-analyzer
            ]
            ++ [
              self.formatter.${system}
              pkgs.nixd
            ];

          # Required by rust-analyzer
          env.RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );

    formatter = forEachSupportedSystem ({pkgs, ...}: pkgs.alejandra);
  };
}
