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

If you use Nix to define your system, you can install it with the default overlay
```nix
{
  nixpkgs.overlays = [
    hyprqtile.overlays.default
  ];
}
```


## Note

This project is still in its early stages. It was inspired by
[hyprnome](https://github.com/donovanglover/hyprnome).
