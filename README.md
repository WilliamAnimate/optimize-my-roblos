<!-- markdownlint-disable MD033 -->

# optimize-my-roblos

A simple and small GUI application for optimizing the roblox ClientAppSettings.json file. Supports both Roblox and Roblox Studio

> [!IMPORTANT]
> Optimize-my-Roblos **DOES NOT** support microsoft store Roblox!

[Optimize my Roblos is based off kit's fflags](https://github.com/catb0x/Roblox-Potato-FFlags)

## Table of contents

- [Goal](#goal)
- [Troubleshooting/manually installing](#troubleshooting)
- [Building from Source](#building-from-source)
- [License](#license)

## Goal

This program has one and only one purpose only: put the flags at the right place. No editing the flags, no persistance across updates.

> **If**
> you need something that can preserve the flags even when Roblox updates **or** have a gui to edit your flags...
>
> consider using [BloxStrap](https://github.com/pizzaboxer/bloxstrap) over Optimize-my-Roblos

## Troubleshooting

If it isn't working, then its probably best to install it manually. Don't worry, it isn't hard.

1. Find where Roblox is installed, this can usually be done by right clicking the Roblox icon and clicking `show in folder`. If it isn't there, check your start menu or search for its location.

1. Create a new folder called `ClientSettings`

1. Create a file called `ClientAppSettings.json`

1. Copy and paste [one of these .json files](https://github.com/WilliamAnimate/optimize-my-roblos/blob/main/src-tauri/src/) into `ClientAppSettings.json`

1. Done.

## Building from Source

**Looking for downloads? It isn't here, look over [here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)**.

### prerequisites

- [Rust Toolchain](https://rust-lang.org)
- [Tauri](https://tauri.app/) (via `cargo install create-tauri-app --locked`)

> [!NOTE]
> The prebuilt binaries are built with the **Nightly Rust Toolchain**

### then, to build

run the `b.bat` file (or run `cargo tauri build`) in the root directory, and to compile in release mode, run `compile.bat`.

## License

The code under this repo is licensed under MIT.

This project uses the Lexend font, which is licensed under OFL.

## Download

**Congratulations!** you've scrolled past it. [Downloads are over here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)
