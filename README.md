aw-watcher-window-wayland
=========================

[![Build](https://github.com/ActivityWatch/aw-watcher-window-wayland/actions/workflows/build.yml/badge.svg)](https://github.com/ActivityWatch/aw-watcher-window-wayland/actions/workflows/build.yml)

An alternative to `aw-watcher-afk` and `aw-watcher-window` that includes support for a number of Wayland compositors and reports supported data to their respective buckets within ActivityWatch.

### Compatibility

| Window Manager | Version | Windows | Idle Timer |
|-----|-----|-----|-----|
| [phosh](https://gitlab.gnome.org/World/Phosh/phosh) | ? | ✔️ | ✔️ |
| [sway](https://swaywm.org/) | >= 1.5 | ✔️ | ✔️ |
| [niri](https://yalter.github.io/niri/) | ? | wip | wip |
| GNOME / [Mutter](https://gitlab.gnome.org/GNOME/mutter) | ? | ❌ | ❌ |
| [Wayfire](https://wayfire.org/) | ? | ? | ? |
| KDE / [KWin](https://invent.kde.org/plasma/kwin) | ? | ❌ | ❌ |

## How to use

1. Start your wayland compositor
2. Start `aw-server` (or `aw-qt` with both `aw-watcher-afk` and `aw-watcher-window` disabled to prevent conflicts)
3. Start `aw-watcher-window-wayland`

If you want to autostart `aw-watcher-window-wayland` without `aw-qt`, you can use the provided [.desktop file](./aw-watcher-window-wayland.desktop).

## How to build

1. Install rust and cargo (any recent stable version is sufficient)
2. Run `cargo build --release`
3. A binary will be built inside the `target/release` folder named `aw-watcher-window-wayland` which can be run

### Dependencies

Make sure to have the following dependencies installed before building the project.

- pkg-config
- openssl-dev (debian libssl1.0-dev)

### Using Nix

If you have Nix installed, you can simply run `nix develop` to enter a configured dev shell.

