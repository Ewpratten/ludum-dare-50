# Ludum Dare 50: *unnamed game*
[![Build Full Release](https://github.com/Ewpratten/ludum-dare-50/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/ludum-dare-50/actions/workflows/build.yml)

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
