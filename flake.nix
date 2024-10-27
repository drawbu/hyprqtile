{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, ... }@inputs:
    inputs.utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        rev = if pkgs.lib.hasAttr "rev" self then self.shortRev else self.dirtyShortRev;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          env = rec {
            RUST_LOG = "info";
            RUSTC_VERSION = pkgs.rustc.version;
            RUSTUP_TOOLCHAIN = RUSTC_VERSION;
          };
          packages = with pkgs; [
            rust-analyzer
            clippy
          ];
        };

        packages = {
          default = self.packages.${system}.hyprqtile;
          hyprqtile = pkgs.rustPlatform.buildRustPackage {
            pname = "hyprqtile";
            version = "0.1.0";
            src = ./.;
            env.GIT_REV = rev;
            cargoLock.lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
