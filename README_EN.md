# Nine Slice Sprite 2D

This document describes the `nine_slice_sprite_2d` Rust GDExtension crate.

The extension registers a `Node2D`-derived Godot class named `NineSliceSprite2D`. It is used to render a 2D nine-slice sprite while keeping the four corners intact and stretching or tiling the edges and center according to configuration.

## Node Info

- Class name: `NineSliceSprite2D`
- Base class: `Node2D`
- GDExtension entry file: `nine_slice_sprite_2d.gdextension`
- Rust entry: `src/lib.rs`
- Main implementation: `src/nine_slice_sprite_2d.rs`

## Features

- Independent patch margins on all four sides
- Optional center drawing
- Three modes for both horizontal and vertical axes
  - `Stretch`
  - `Tile`
  - `Tile Fit`
- Supports `region_rect`
- Supports `centered`, `offset`, `flip_h`, and `flip_v`
- Supports sprite sheet frame control
  - `hframes`
  - `vframes`
  - `frame`
  - `frame_coords`
- Exposes signals
  - `frame_changed`
  - `texture_changed`
  - `resized`

## Directory Layout

```text
Cargo.toml
src/
  lib.rs
  nine_slice_sprite_2d.rs
nine_slice_sprite_2d.gdextension
target/
README.md
README_EN.md
```

## Build

Run this from the current directory:

```bash
cargo build
```

Debug artifacts are written to `target/debug/`.

Common output files:

- Windows: `target/debug/nine_slice_sprite_2d.dll`
- Linux: `target/debug/libnine_slice_sprite_2d.so`
- macOS: `target/debug/libnine_slice_sprite_2d.dylib`

Release build:

```bash
cargo build --release
```

## Loading in Godot

Godot loads the extension through `nine_slice_sprite_2d.gdextension` in this directory.

That file already defines library paths for:

- `windows.debug.x86_64`
- `windows.release.x86_64`
- `linux.debug.x86_64`
- `linux.release.x86_64`
- `macos.debug`
- `macos.release`
- `macos.debug.arm64`
- `macos.release.arm64`
- `android.debug.arm64`
- `android.release.arm64`

The declared minimum compatibility version is Godot `4.1`.

## Usage

1. Build the dynamic library for your platform.
2. Load `nine_slice_sprite_2d.gdextension` in a Godot project.
3. Create a `NineSliceSprite2D` node.
4. Set `texture`, `size`, and the four `patch_margin_*` properties.
5. Configure axis modes and frame-related properties as needed.

## Main Properties

- `texture`: source texture
- `size`: target draw size
- `draw_center`: whether the center area is drawn
- `region_rect`: source sampling rectangle
- `patch_margin_left`
- `patch_margin_top`
- `patch_margin_right`
- `patch_margin_bottom`
- `axis_stretch_horizontal`
- `axis_stretch_vertical`
- `centered`
- `offset`
- `flip_h`
- `flip_v`
- `hframes`
- `vframes`
- `frame`
- `frame_coords`

## Main Methods

- `get_patch_margin(margin: Side) -> int`
- `set_patch_margin(margin: Side, value: int)`

The Rust implementation also contains:

- draw logic in `draw()`
- signal emission and property update wiring
- two-way synchronization between `frame` and `frame_coords`

## Dependencies

Current `Cargo.toml` dependency:

```toml
[dependencies]
godot = "0.4.5"
```

## Development Notes

- The extension entry point is registered in `src/lib.rs` through `#[gdextension]`
- `NineSliceSprite2D` is exposed to Godot through `#[derive(GodotClass)]`
- The class uses `#[class(base = Node2D, tool)]`, so it works in the editor
- Nine-slice rendering is performed through `RenderingServer::canvas_item_add_nine_patch`
