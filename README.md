# adhocrays

> [!WARNING]
> This is a hobby project. New features and maintenance to be done at my whimsy.

[Raylib](https://github.com/raysan5/raylib) bindings in Rust, thrown together for my own projects.
It is recommended that you make your own bindings or use something that is properly maintained.

## Build from Source

To clone the repository, run:
```
git clone https://github.com/FallibleVagrant/adhocrays --recurse-submodules --shallow-submodules
```
Raylib is set as a submodule of the repo. The command downloads a specific commit of raylib, to a depth of one.

Alternatively, you may clone this repo as usual and run:
```
git submodule update --init --depth 1
```
It achieves the same effect.

You need `make` and `cp` installed to build this project. Also cargo.

The build script will need to be modified for Windows. I don't have a machine to test on.

Building may appear to hang since cargo eats all output from build scripts. No, there doesn't appear to be a way to fix this.

Once the repo is set up, simply call:
```
cargo build
```

## Licensing
Raylib is available under the zlib/libpng license.
I release the bindings themselves under the same.
