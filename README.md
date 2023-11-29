<!-- markdownlint-disable MD033 -->

# optimize-my-roblos

A simple and small GUI application for optimizing the roblox ClientAppSettings.json file.

[Optimize my Roblos is based off kit's fflags](https://rentry.co/robloxpotatofflags)

## Table of contents

- [Why Optimize_my_Roblos over others?](#why-optimize_my_roblos-over-others)
- [Why others over Optimize_my_Roblos?](#why-others-over-optimize_my_roblos)
- [Troubleshooting/manually installing](#troubleshooting)
- [Building from Source](#building-from-source)
- [License](#license)

## Why Optimize_my_Roblos over others?

- Actually has a GUI.

- Some applications that promise to do the same thing as Optimize_my_Roblos are really, really big binaries, some of them being 20 megabytes; Optimize_my_Roblos is **less** than 3 megabytes.

- No need to stay in the background.

- Fast: less than 5 seconds of your time and you're good to go.

- 100% Open Source Software, powered by Open Source. No hidden malware.

## Why others over Optimize_my_Roblos?

<!-- markdownlint-disable MD052 -->
- Literally just a website.
<!-- markdownlint-enable MD052 -->

- Made by someone who has no clue what a fflag stands for.

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

you may or may not need the following:

- [npm](https://nodejs.org) <!-- wait isn't this techinically nodejs? am i high? -->
- [**NIGHTLY** rust toolchain](https://rust-lang.org)

> [!NOTE]
> Versions above 1.0.0 are built using the **Rust nightly toolchain**, `rustc 1.76.0-nightly (2c1b65ee1 2023-11-11)`
>
> (2,695,680 bytes -> 2,575,872 bytes)

### then, to build

run the `b.bat` file (or run `npm run tauri dev`) in the root directory, and to compile in release mode, run `npm run tauri build`, still in the root directory.

## License

This project is licensed under MIT. This means that you are allowed to do anything you want with this project, as long as you do the following:

1. Give me attribution for using the code.
1. Understand that no warranty will be provided, so I will not be liable if this code somehow wipes all the databases of your startup or something.

This project uses fonts licensed under the OFL.

## Download

**Congratulations!** you've scrolled past it. [Downloads are over here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)
