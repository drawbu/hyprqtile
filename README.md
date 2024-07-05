# HyprQtile

[Qtile](https://qtile.org)-like workspace switcher for
[Hyprland](https://hyprland.org).


## Usage

```sh
# Move to a workspace, and if it's on another monitor,
# swap the two workspace's assigned monitors.
hyprqtile move 2
```


## Installation

```sh
# With Nix
nix build

# Without Nix
cargo build --release
```

`~/.config/hypr/hyprland.conf`
```conf
bind=SUPER shift, 1, exec, hyprqtile move 1
bind=SUPER shift, 2, exec, hyprqtile move 2
bind=SUPER shift, 3, exec, hyprqtile move 3
...
```

If you use home-manager with nix, you can install it
```nix
# flake.nix
{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.hyprqtile.url = "github:drawbu/hyprqtile";
  # this line assume that you also have nixpkgs as an input
  inputs.hyprqtile.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { nixpkgs, hyprqtile, ... }: {
    # replace `myhostname` with your actual hostname
    nixosConfigurations.myhostname = nixpkgs.lib.nixosSystem rec {
      system = "x86_64-linux";
      extraSpecialArgs = {
        inherit (hyprqtile.packages.${system}) hyprqtile;
      };
    };
  };
}
```


## Note

This project is still in its early stages. It was inspired by
[hyprnome](https://github.com/donovanglover/hyprnome).
