{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          cargoToml = lib.fromTOML ./cargo.toml;
          toolchain = pkgs.fenix.fromToolchainFile {
            dir = ./.;
          };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (inputs.fenix.overlays.default)
            ];
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              openssl # Only required for OpenSSL Feature
              rust-analyzer
              toolchain
            ];
          };

          packages = rec {
            default =
              (pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              }).buildRustPackage
                {
                  pname = "curl-rs";
                  version = cargoToml.package.version;
                  src = ./.;

                  cargoLock.lockFile = ./Cargo.lock;
                };
            curl-rs = default;
            curl-rs-compat = default.overideAttrs (
              oldAttrs: with pkgs; {
                buildInputs = lib.optionals stdenv.hostPlatform.isLinux [ openssl ];
                cargoBuildFlags = [ "--features=compatibility" ];
              }
            );
          };
        };
    };
}
