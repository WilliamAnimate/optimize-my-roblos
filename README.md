<!-- markdownlint-disable MD033 -->

# optimize-my-roblos

A simple and small GUI application for optimizing the roblox ClientAppSettings.json file.

## Table of contents

- [Why Optimize_my_Roblos over others?](#why-optimize_my_roblos-over-others)
- [Why others over Optimize_my_Roblos?](#why-others-over-optimize_my_roblos)
- [How?](#how)
- [Troubleshooting/manually installing](#troubleshooting)
- [Building from Source](#building-from-source)
- [Legal](#legal)

## Why Optimize_my_Roblos over others?

- Actually has a GUI.

- Some applications that promise to do the same thing as Optimize_my_Roblos are really, really big binaries, some of them being 20 megabytes; Optimize_my_Roblos is **less** than 3 megabytesa.

- Does not need/have an option to stay open in the background, no need to lose that performance for no reason.

- Backend is written in [Rust](https://rust-lang.org), the language that even Microsoft themselves are adopting.

- 100% Open Source Software, powered by Open Source. No hidden malware... unlike some other ones...

## Why others over Optimize_my_Roblos?

- ~~Optimize_my_Roblos is way too bloated lmao.~~

<!-- markdownlint-disable MD052 -->
- written in the Rust and [object][Object] programming language, when the others are written in superior languages like batch, c++, and others.
<!-- markdownlint-enable MD052 -->

- made by someone who has no clue what a fflag stands for.

## How?

> [!NOTE]
> I'm no expert in this field, take what I say with a grain of salt.

Roblox offers some hidden features that allow you to tweak how the client works, to a certain extent. Most notably for this project, are **the renderer**, **target frame rate**, **Telemetry Control**. This is known as the `ClientAppSettings.json` file.

If done correctly, you can make roblox significantly faster than what they give you by default, no game files modified, no threads injected, and thus, is an anticheat friendly way to optimize the game... probably, hopefully.

## Troubleshooting

If it isn't working, then its probably best to install it manually. Don't worry, it isn't hard.

1. Find where Roblox is installed, this can usually be done by right clicking the Roblox icon and clicking `show in folder`. If it isn't there, check your start menu or search for its location.

1. Create a new folder called `ClientSettings`

1. Create a file called `ClientAppSettings.json`

1. Copy and paste [this code](https://github.com/WilliamAnimate/optimize-my-roblos/blob/main/src-tauri/src/ClientAppSettings.json) into `ClientAppSettings.json`

1. Win your games because you aren't playing on what looks like a slideshow

## Building from Source

**Looking for downloads? It isn't here, look over [here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)**.

### prerequisites

you may or may not need the following:

- [npm](https://nodejs.org) <!-- wait isn't this techinically nodejs? am i high? -->
- [**NIGHTLY** rust toolchain](https://rust-lang.org)

> [!NOTE]
> Versions above 1.0.0 are built using the **Rust nightly toolchain**, `rustc 1.76.0-nightly (2c1b65ee1 2023-11-11)`
> this is so I can have a locally-compiled `std`. (2,695,680 bytes -> 2,575,872 bytes)

### then, to build

run the `b.bat` file (or run `npm run tauri dev`) in the root directory, and to compile in release mode, run `npm run tauri build`, still in the root directory.

### Optimizing binary size

*wait wait wait*, why are you optimizing for size? this is all about performance!

well, yes, but performance is already *quite fast* with Rust, and we don't need an extra nanosecond.

<hr>

before building in release mode, I like to use `cargo bloat` and `unused-features analyze` then `unused-features prune`. This ensures that rustc has less to compile.

most of the time, opt-level set to `"z"` are smaller than `"s"` and `3`, but always play around to find out which ones are smaller.

for all of my builds, disabling the default `compression` tauri feature results in a smaller binary size, by about ~335872 bytes (first run ever without `compression`)

### fixing compile errors

learn [the way of the rust](https://doc.rust-lang.org).

## Legal

This project is licensed under MIT. This means that you are allowed to do anything you want with this project, as long as you do the following:

1. Give me attribution for using the code.
1. Understand that no warranty will be provided, so I will not be liable if this code somehow wipes all the databases of your startup or something.

that's all.

## Download

**Congratulations!** you've scrolled past it. [Downloads are over here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)
