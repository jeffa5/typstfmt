{
  description = "typstfmt";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          overlays = [rust-overlay.overlays.default];
          inherit system;
        };
        lib = pkgs.lib;
        rust = pkgs.rust-bin.nightly.latest.default;
        cargoNix = import ./Cargo.nix {inherit pkgs;};
        workspacePackages = lib.attrsets.mapAttrs (name: value: value.build) cargoNix.workspaceMembers;
      in {
        packages = workspacePackages;

        formatter = pkgs.alejandra;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust.override {extensions = ["rust-src"];})
            cargo-watch
            cargo-fuzz
            crate2nix
          ];
        };
      }
    );
}
