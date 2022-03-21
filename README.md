# Ludum Dare 50: *unnamed game*
[![Build Full Release](https://github.com/Ewpratten/ludum-dare-50/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-50/actions/workflows/build.yml)

## Navigating this repository

- `/game`: The game resource directory
  - `/game/desktop_wrapper`: A desktop launcher for the game
  - `/game/game_logic`: The game code
  - `/game/dist`: The assets for the game (these are packaged with the final executable)
- `/assets`: Various asset files sorted by user (these are **not** packaged with the game)
- `/docs`: Documentation for the game
- `/automation`: Tools to make our lives easier. Written in various languages
- `/third_party`: Custom forks of third party libraries

## Cloning

**IMPORTANT:** This project makes use of recursive submodules. Make sure to pull them via GitKracken, or with the following command:

```sh
git submodule update --init --recursive
```
*Your builds will fail unless this is done*

## Development notes

When working on the settings and savestate file code, there is a chance you will corrupt your save files. If this happens, launch the game with the following command to fix them:

```sh
cargo run -- --force-recreate-savefiles
```
