[![Rust Build and Test](https://github.com/QueenOfSquiggles/Squiggles-Core/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/QueenOfSquiggles/Squiggles-Core/actions/workflows/rust.yml)

# Squiggles-Core
An opinionated utility library for making 3D games in Godot 4 written in rust.

# Main Features
- Virtual Camera system
- Hierarchical Finite State Machine structure
    - Will be rewritten once support for abstract functions is added to GDExtension
- Interaction System:
    - Detect from RayCast3D
    - Detect from Area3D
    - Interactble Area3D
    - Interactble RigidBody3D
    - Interactble StaticBody3D
    - Interactble CharacterBody3D
-


# Platform Support

## Batteries Included
Squiggles Core by default is built to target "the big three" desktop platforms (Windows, Mac, and Linux). The way GDExtensions work is they must be built for a Debug context and for a Release context. The Debug files are used during Debug builds of your game/software as well as in the editor!

Platforms:
- Windows (64 bit)
- Mac (Apple Darwin)
- Linux (64 bit)


## Build it yourself
Because Squiggles Core is written in rust and only uses rust-compatible dependencies, you can easily compile alternate versions of this library yourself so long as it is a platform supported by [cross-rs](https://github.com/cross-rs/cross).

The easist way to compile yourself is to be on a linux platform. Either natively or through Windows Subsystem Linux (WSL). From there you can run the `build_release.sh` script to autmatically compile all supported versions as well as packaging the important files into a single zip archive.

Dependencies:
- `cross-rs`
- `podman` or rootless `docker`
- `zip`

### Compiling for unsupported platforms
If you want to support different platforms such as Mobile you will need to compile "by hand" which uses the same dependencies as `build_release.sh`. Read through cross-rs's documentation for how to compile for your target architecture.
Then you can add the necessary files into the `target` folder of your install and set up the `squiggles_core.gdextension` file to include your library files. Refer to the [godot docs on this](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_cpp_example.html#using-the-gdextension-module) for more information on what keys mean what platforms
