<!-- markdownlint-disable MD033 -->

# optimize-my-roblos

A simple, low overhead & file footprint GUI application for optimizing the roblox ClientAppSettings.json file.

## Table of contents

- [Why Optimize_my_Roblos over others?](#why-optimize_my_roblos-over-others)
- [Why others over Optimize_my_Roblos?](#why-others-over-optimize_my_roblos)
- [How?](#how)
- [Building from Source](#building-from-source)

## Why Optimize_my_Roblos over others?

<!-- - Not a console gui (but its possible to use CLI mode, if you really want to.) -->

- Actually has a GUI.

- Does not & never stays open in the background, reducing your machine's risk of attack by having to exploit OTHER processes.

- Backend is written in [Rust](https://rust-lang.org), meaning that even IF this process is open, it'll be extremely difficult to exploit.

- Powered by 100% (F)(L)OSS. No hidden keyloggers.

## Why others over Optimize_my_Roblos?

- ~~Optimize_my_Roblos is way too bloated lmao~~

## How?

> [!NOTE]
> I'm no expert in this field, take what I say with a grain of salt.

Roblox offers some hidden features that allow you to tweak how the client works, to a certain extent. Most notably for this project, are **the renderer**, **target frame rate**, **Telemetry Control**. This is known as the `ClientAppSettings.json` file.

If done correctly, you can make roblox significantly faster than what they give you by default, no game files modified, no threads injected, and thus, is an anticheat friendly way to optimize the game... probably, hopefully.

## Building from Source

**Looking for downloads? It isn't here, look over [here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)**.

### prerequisites

you may or may not need the following:

- [npm](https://nodejs.org) <!-- wait isn't this techinically nodejs? am i high? -->
- [rust toolchain](https://rust-lang.org)

### then, to build

run the `b.bat` file (or run `npm run tauri dev`) in the root directory, and to compile in release mode, run `npm run tauri build`, still in the root directory.

### Optimizing binary size

*wait wait wait*, why are you optimizing for size? this is all about performance!

well, yes, but performance is already *quite fast* with Rust, and we don't need an extra nanosecond.

<hr>

before building in release mode, i like to use `cargo bloat` and `unused-features analyze`, but unused-features analyze probably does (some of) the work for you.

for early debug builds, opt-level set to `"z"` are smaller than `"s"` and `3`

for all of my builds, disabling the default `compression` tauri feature results in a smaller binary size, by about ~335872 bytes (first run ever without `compression`)

so you can only play around and find out.

### fixing compile errors

learn [the way of the rust](https://doc.rust-lang.org).

## Legal

This project is licensed under MIT. This means that you are allowed to do anything you want with this project, as long as you do the following:

1. Give me attribution for using the code.
1. Understand that no warranty will be provided, so I will not be liable if this code somehow wipes all the databases of your startup or something.

that's all.

## Download

**Congratulations!** you've scrolled past it. [Downloads are over here](https://github.com/WilliamAnimate/optimize-my-roblos/releases)
