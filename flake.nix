{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, ... }@inputs:
    let
      forAllSystems =
        function:
        inputs.nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: function (import inputs.nixpkgs { inherit system; }));
    in
    {
      packages = forAllSystems (pkgs: {
        default = self.packages.${pkgs.system}.hyprqtile;
        hyprqtile = pkgs.rustPlatform.buildRustPackage {
          name = "hyprqtile";
          src = ./.;
          env.BUILD_REV = "${self.rev or self.dirtyShortRev}-git";
          cargoLock.lockFile = ./Cargo.lock;
        };
      });

      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) hyprqtile;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${pkgs.system};
          env.RUSTUP_TOOLCHAIN = pkgs.rustc.version;
          packages = with pkgs; [ rustup ];
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-tree);
    };
}
