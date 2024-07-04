{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, ...} @ inputs:
    inputs.utils.lib.eachDefaultSystem (system: let
      pkgs = inputs.nixpkgs.legacyPackages.${system};
    in {
      formatter = pkgs.alejandra;

      devShells.default = pkgs.mkShell {
        RUSTC_VERSION = "stable";
        packages = with pkgs; [rustup];
      };

      packages = {
        default = self.packages.${system}.hyprqtile;
        hyprqtile = pkgs.rustPlatform.buildRustPackage {
          pname = "hyprqtile";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      };
    });
}
