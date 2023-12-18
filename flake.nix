{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs = { self, nixpkgs, rust-overlay, systems, ... }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);

      mkLipsumCli = { rustPlatform, lib, ... }: rustPlatform.buildRustPackage rec {
        pname = "lipsum-cli";
        version = "0.3.0";

        src = builtins.path {
          path = ./.;
          name = pname;
        };
        cargoLock.lockFile = ./Cargo.lock;

        meta = {
          description = "A small terminal application written in Rust language. ";
          homepage = "https://github.com/xlith/lipsum-cli";
          license = lib.licenses.mit;
        };
      };
    in
    {
      apps = eachSystem (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/lipsum-cli";
        };
      });

      packages = eachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          default = pkgs.callPackage mkLipsumCli { };
        });

      overlays.default = final: prev: {
        lipsum-cli = prev.callPackage mkLipsumCli { };
      };

      devShells = eachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;

            overlays = [ rust-overlay.overlays.default ];
          };

          toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
        in
        {
          default = pkgs.mkShell {
            packages = [ toolchain ];
          };
        });
    };
}

