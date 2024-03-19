[![Rust Build and Test](https://github.com/QueenOfSquiggles/Squiggles-Core/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/QueenOfSquiggles/Squiggles-Core/actions/workflows/rust.yml)

# Sqore (formerly Squiggles-Core)
An opinionated utility library for making 3D games in Godot 4.2 written in rust.

# Mission

This is my tool library that I use for making games. The goal of this library is to make it easier to make the specific kinds of games that I like to make, 3D games with a focus on narrative. As I need/want more tools from this I will add them. It's entirely possible I will forget to add new features to this list

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
- Global Game Settings (Accessed as `CoreGlobals.config`)
    - Graphics
        - Reasonable defaults for 3D setttings
        - `WorldEnvironmentSettingsCompliant` custom WorldEnvironment that automatically replicates settings from the graphics settings
    - Audio: Volume controls for all available audio busses
    - Controls: Support for controls remapping with any generic inputs
    - Gameplay: Custom values & types serialized for you
- Full dialog system
    - JSON format (easily edit from within Godot)
    - blackboard with simple scripting and querying
        - setting variables as well as add/sub on number types
        - arbitrary queries including all comparative operators
        - templating using `{{ var_name }}` within Character Names and dialog Text
        - GDScript access
            - query values
            - perform script actions same as within dialog files
    - signal with specific name and argument array
    - choices with optional requirements and arbitrary actions upon selection
    - customize appear and hide tweening
    - customize words per minute for text appearing (default is 150 WPM, my preference is 500 WPM)
- staticly typed for easy interfacing with autocomplete in GDScript.
- `InputAxisAllocator` utility for collecting axis movement
    - Joystick axis vector
    - Mouse motion
    - Independant contribution scaling
- Fully open source



# Usage Examples:

Run dialog from file with callback for track ending
```gdscript
signal request_player_can_move(can_move: bool)

CoreDialog.load_track_file(file_name)
CoreDialog.event_bus.track_ended.connect( \
    Callable(self, "emit_signal") \
    .bind("request_player_can_move", true), CONNECT_DEFERRED | CONNECT_ONE_SHOT)
```

Force reload of Core graphics settings. This includes changing the windowing mode and main viewport scaling mode (supports AMD FSR)
```
CoreGlobals.config.graphics.mark_dirty()
```

Minimal FiniteState class
```gdscript
extends FiniteState

func on_enter() -> void:
    pass

func on_exit() -> void:
	pass

func tick(_delta : float) -> void:
	pass

```
> Note that as of 4.2 an internal and partially janky system is used to call the appropriate methods. As of Godot 4.3, I can define virtual functions which can be overridden from GDScript. So once 4.3 is out I will start adapting which should make the overall FSM system more ergonomic


# Platform Support

Sqore supports godot 4.2.X, but I generally test with whatever is the latest stable release of Godot.

## Batteries Included
Sqore by default is built to target "the big three" desktop platforms (Windows, Mac, and Linux). The way GDExtensions work is they must be built for a Debug context and for a Release context. The Debug files are used during Debug builds of your game/software as well as in the editor!

Platforms:
- Windows (64 bit)
- Mac (Apple Darwin)
- Linux (64 bit)


## Build it yourself
Because Sqore is written in rust and only uses rust-compatible dependencies, you can easily compile alternate versions of this library yourself so long as it is a platform supported by [cross-rs](https://github.com/cross-rs/cross).

The easist way to compile yourself is to be on a linux platform. Either natively or through Windows Subsystem Linux (WSL). From there you can run the `build_release.sh` script to autmatically compile all supported versions as well as packaging the important files into a single zip archive.

Dependencies:
- `cross-rs`
- `podman` or rootless `docker`
- `zip`

### Compiling for unsupported platforms
If you want to support different platforms such as Mobile you will need to compile "by hand" which uses the same dependencies as `build_release.sh`. Read through cross-rs's documentation for how to compile for your target architecture.
Then you can add the necessary files into the `target` folder of your install and set up the `sqore.gdextension` file to include your library files. Refer to the [godot docs on this](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/gdextension_cpp_example.html#using-the-gdextension-module) for more information on what keys mean what platforms
