{
  description = "typstfmt";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crate2nix.url = "github:jeffa5/crate2nix";
  };

  outputs = {
    self,
    nixpkgs,
    crate2nix,
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
    };
    lib = pkgs.lib;
    cargoNix = import ./Cargo.nix {inherit pkgs;};
    workspacePackages = lib.attrsets.mapAttrs (name: value: value.build) cargoNix.workspaceMembers;
  in {
    packages.${system} =
      workspacePackages
      // {
        default = workspacePackages.typstfmt;
      };

    overlays.default = final: prev: {
      typstfmt = self.packages.${prev.system}.typstfmt;
    };

    formatter.${system} = pkgs.alejandra;

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs;
        [
          rustc
          cargo
          rustfmt
          clippy

          cargo-watch
          cargo-fuzz
        ]
        ++ [
          crate2nix.packages.${system}.crate2nix
        ];
    };
  };
}
