# Nine Slice Sprite 2D

这是 `nine_slice_sprite_2d` Rust GDExtension crate 的说明文档。

该扩展向 Godot 注册一个 `Node2D` 派生节点 `NineSliceSprite2D`，用于在 2D 场景中绘制九宫格贴图。它会保留四角区域不变，并按配置对边缘和中心区域进行拉伸或平铺。

## 节点信息

- 类名：`NineSliceSprite2D`
- 基类：`Node2D`
- GDExtension 入口：`nine_slice_sprite_2d.gdextension`
- Rust 入口：`src/lib.rs`
- 主要实现：`src/nine_slice_sprite_2d.rs`

## 功能特性

- 支持四边独立 `patch margin`
- 支持中心区域绘制开关
- 支持横向和纵向三种模式
  - `Stretch`
  - `Tile`
  - `Tile Fit`
- 支持 `region_rect`
- 支持 `centered`、`offset`、`flip_h`、`flip_v`
- 支持精灵表帧控制
  - `hframes`
  - `vframes`
  - `frame`
  - `frame_coords`
- 提供信号
  - `frame_changed`
  - `texture_changed`
  - `resized`

## 目录结构

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

## 构建

在当前目录执行：

```bash
cargo build
```

调试构建产物会输出到 `target/debug/`。

常见平台对应文件：

- Windows: `target/debug/nine_slice_sprite_2d.dll`
- Linux: `target/debug/libnine_slice_sprite_2d.so`
- macOS: `target/debug/libnine_slice_sprite_2d.dylib`

发布构建：

```bash
cargo build --release
```

## 在 Godot 中加载

Godot 通过同目录下的 `nine_slice_sprite_2d.gdextension` 加载本扩展。

该文件已配置以下目标路径：

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

最低兼容版本声明为 Godot `4.1`。

## 使用方式

1. 确保动态库已经构建完成。
2. 在 Godot 项目中加载 `nine_slice_sprite_2d.gdextension`。
3. 创建 `NineSliceSprite2D` 节点。
4. 设置 `texture`、`size` 和四个 `patch_margin_*` 属性。
5. 视需要设置横纵方向的拉伸模式与动画帧属性。

## 主要属性

- `texture`：源纹理
- `size`：目标绘制尺寸
- `draw_center`：是否绘制中心区域
- `region_rect`：纹理采样区域
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

## 主要方法

- `get_patch_margin(margin: Side) -> int`
- `set_patch_margin(margin: Side, value: int)`

Rust 实现内部还包含：

- 绘制逻辑 `draw()`
- 信号发射与属性联动
- `frame` 与 `frame_coords` 的双向同步

## 依赖

当前 `Cargo.toml` 使用：

```toml
[dependencies]
godot = "0.4.5"
```

## 开发说明

- 扩展入口通过 `#[gdextension]` 在 `src/lib.rs` 中注册
- `NineSliceSprite2D` 使用 `#[derive(GodotClass)]` 暴露给 Godot
- 节点为 `#[class(base = Node2D, tool)]`，可在编辑器中工作
- 九宫格绘制通过 `RenderingServer::canvas_item_add_nine_patch` 完成
