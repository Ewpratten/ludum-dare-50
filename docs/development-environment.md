# Development Environment

## Prerequisite Tooling

On all systems, you must have [Rust](https://www.rust-lang.org/tools/install), [git](https://git-scm.com/), [Python 3](https://www.python.org/) (with `pip`), and [cmake](https://cmake.org/download/) installed.

### Additional Dependencies for Linux

Linux systems require some additional development headers:

**Ubuntu:**

```sh
sudo apt install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev
```

**Fedora:**

```sh
sudo dnf install alsa-lib-devel mesa-libGL-devel libX11-devel libXrandr-devel libXi-devel libXcursor-devel libXinerama-devel
```

## Cloning the Repository

If you are using [GitHub Desktop](https://desktop.github.com/) or have a [GitKraken](https://www.gitkraken.com/) Pro license, just clone this repo as usual.

If you are cloning via the CLI, you will need an additional step to ensure our forked dependencies are pulled in correctly:

```sh
git clone https://github.com/Ewpratten/ludum-dare-50
cd ludum-dare-50
git submodule update --init --recursive

# Optionally, pull in the dependencies for the artist tools
python3 -m pip install -r requirements.txt
```

## First Build

We make heavy use of `cargo` to handle dependency management and build configuration. To build the game, just run:

```sh
cargo build
```

This will take quite a while on first build as it will download all dependencies and compile them locally.
