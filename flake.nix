{
  description = "Env";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.treefmt-nix.url = "github:numtide/treefmt-nix";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      treefmt-nix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = "latest";
        rustChannel = "nightly";
        # rustChannel = "stable";
        #rustChannel = "stable";
        #rustVersion = "1.62.0";
        rust = pkgs.rust-bin.${rustChannel}.${rustVersion}.default.override {
          extensions = [
            "rust-src" # for rust-analyzer
          ];
        };

        treefmtConfig = {
          # Used to find the project root
          projectRootFile = "flake.nix";
          programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            taplo.enable = true;
          };
          settings.global.excludes = [
            "target/**"
          ];
        };
      in
      {
        formatter = treefmt-nix.lib.mkWrapper pkgs treefmtConfig;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
          ]
          ++ (with pkgs; [
            protobuf
            llvmPackages.bintools
            bashInteractive
            rust-analyzer
            rustc
            cargo-edit
            cargo-machete
            cargo-watch
            cargo-deny
            cargo-nextest
            taplo
            watchexec
            bacon
          ]);
          shellHook = ''
            export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
            export PATH=$PATH:${pkgs.uv}/bin
          '';
        };
      }
    );
}
