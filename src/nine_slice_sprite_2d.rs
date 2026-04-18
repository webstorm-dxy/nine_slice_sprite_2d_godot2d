use godot::builtin::Side;
use godot::classes::{rendering_server::NinePatchAxisMode, RenderingServer, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node2D, tool)]
struct NineSliceSprite2D {
    base: Base<Node2D>,

    #[export]
    #[var(get = get_texture, set = set_texture)]
    texture: Option<Gd<Texture2D>>,

    #[export]
    #[var(get = get_size, set = set_size)]
    size: Vector2,

    #[export]
    #[var(get = is_draw_center_enabled, set = set_draw_center)]
    draw_center: bool,

    #[export]
    #[var(get = get_region_rect, set = set_region_rect)]
    region_rect: Rect2,

    #[export_group(name = "Patch Margin", prefix = "patch_margin_")]
    #[export]
    #[var(get = get_patch_margin_left, set = set_patch_margin_left)]
    patch_margin_left: i32,

    #[export]
    #[var(get = get_patch_margin_top, set = set_patch_margin_top)]
    patch_margin_top: i32,

    #[export]
    #[var(get = get_patch_margin_right, set = set_patch_margin_right)]
    patch_margin_right: i32,

    #[export]
    #[var(get = get_patch_margin_bottom, set = set_patch_margin_bottom)]
    patch_margin_bottom: i32,

    #[export_group(name = "Axis Stretch", prefix = "axis_stretch_")]
    #[export(enum = (Stretch, Tile, TileFit))]
    #[var(get = get_h_axis_stretch_mode, set = set_h_axis_stretch_mode)]
    axis_stretch_horizontal: AxisStretchMode,

    #[export]
    #[var(get = get_v_axis_stretch_mode, set = set_v_axis_stretch_mode)]
    axis_stretch_vertical: AxisStretchMode,

    #[export_group(name = "Offset")]
    #[export]
    #[var(get = is_centered, set = set_centered)]
    centered: bool,

    #[export]
    #[var(get = get_offset, set = set_offset)]
    offset: Vector2,

    #[export]
    #[var(get = is_flipped_h, set = set_flip_h)]
    flip_h: bool,

    #[export]
    #[var(get = is_flipped_v, set = set_flip_v)]
    flip_v: bool,

    #[export_group(name = "Animation")]
    #[export]
    #[var(get = get_hframes, set = set_hframes)]
    hframes: i32,

    #[export]
    #[var(get = get_vframes, set = set_vframes)]
    vframes: i32,

    #[export]
    #[var(get = get_frame, set = set_frame)]
    frame: i32,

    #[export]
    #[var(get = get_frame_coords, set = set_frame_coords)]
    frame_coords: Vector2i,
}

#[godot_api]
impl INode2D for NineSliceSprite2D {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            texture: None,
            size: Vector2::ZERO,
            draw_center: true,
            region_rect: Rect2::new(Vector2::ZERO, Vector2::ZERO),
            patch_margin_left: 0,
            patch_margin_top: 0,
            patch_margin_right: 0,
            patch_margin_bottom: 0,
            axis_stretch_horizontal: AxisStretchMode::AxisStretchModeStretch,
            axis_stretch_vertical: AxisStretchMode::AxisStretchModeStretch,
            centered: true,
            offset: Vector2::ZERO,
            flip_h: false,
            flip_v: false,
            hframes: 1,
            vframes: 1,
            frame: 0,
            frame_coords: Vector2i::ZERO,
        }
    }

    fn draw(&mut self) {
        let Some(texture) = self.texture.as_ref() else {
            return;
        };

        let mut source = self.region_rect;
        let texture_size = texture.get_size();
        if source.size == Vector2::ZERO {
            source.size = texture_size / Vector2::new(self.hframes as f32, self.vframes as f32);
        }

        source.position += source.size * self.frame_coords.cast_float();
        source.position = clamp_vec2(source.position, Vector2::ZERO, texture_size - source.size);

        let mut target = Rect2::new(Vector2::ZERO, self.size);
        if self.centered {
            target.position -= self.size / 2.0;
        }

        let mut transform = Transform2D::IDENTITY;
        let mut offset_sign = Vector2::new(1.0, 1.0);

        if self.flip_h {
            transform = transform * Transform2D::FLIP_X;
            offset_sign.x = -1.0;

            if !self.centered {
                target.position.x -= self.size.x;
            }
        }

        if self.flip_v {
            transform = transform * Transform2D::FLIP_Y;
            offset_sign.y = -1.0;

            if !self.centered {
                target.position.y -= self.size.y;
            }
        }

        target.position += self.offset * offset_sign;

        let mut rendering_server = RenderingServer::singleton();
        let canvas_item = self.base().get_canvas_item();
        rendering_server.canvas_item_add_set_transform(canvas_item, transform);
        rendering_server
            .canvas_item_add_nine_patch_ex(
                canvas_item,
                target,
                source,
                texture.get_rid(),
                Vector2::new(self.patch_margin_left as f32, self.patch_margin_top as f32),
                Vector2::new(self.patch_margin_right as f32, self.patch_margin_bottom as f32),
            )
            .x_axis_mode(self.axis_stretch_horizontal.to_rendering_server())
            .y_axis_mode(self.axis_stretch_vertical.to_rendering_server())
            .draw_center(self.draw_center)
            .done();
    }
}

#[godot_api]
impl NineSliceSprite2D {
    #[signal]
    fn frame_changed();

    #[signal]
    fn texture_changed();

    #[signal]
    fn resized();

    #[func]
    fn get_patch_margin(&self, margin: Side) -> i32 {
        match margin {
            Side::LEFT => self.patch_margin_left,
            Side::TOP => self.patch_margin_top,
            Side::RIGHT => self.patch_margin_right,
            Side::BOTTOM => self.patch_margin_bottom,
        }
    }

    #[func]
    fn set_patch_margin(&mut self, margin: Side, value: i32) {
        match margin {
            Side::LEFT => self.set_patch_margin_left(value),
            Side::TOP => self.set_patch_margin_top(value),
            Side::RIGHT => self.set_patch_margin_right(value),
            Side::BOTTOM => self.set_patch_margin_bottom(value),
        }
    }

    #[func]
    fn get_texture(&self) -> Option<Gd<Texture2D>> {
        self.texture.clone()
    }

    #[func]
    fn set_texture(&mut self, texture: Option<Gd<Texture2D>>) {
        self.texture = texture;
        self.signals().texture_changed().emit();
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_size(&self) -> Vector2 {
        self.size
    }

    #[func]
    fn set_size(&mut self, size: Vector2) {
        self.size.x = size
            .x
            .max((self.patch_margin_left + self.patch_margin_right) as f32);
        self.size.y = size
            .y
            .max((self.patch_margin_top + self.patch_margin_bottom) as f32);

        self.emit_item_rect_changed_and_redraw();
        self.signals().resized().emit();
    }

    #[func]
    fn is_draw_center_enabled(&self) -> bool {
        self.draw_center
    }

    #[func]
    fn set_draw_center(&mut self, enabled: bool) {
        self.draw_center = enabled;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn get_region_rect(&self) -> Rect2 {
        self.region_rect
    }

    #[func]
    fn set_region_rect(&mut self, region_rect: Rect2) {
        self.region_rect = region_rect;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn set_patch_margin_left(&mut self, patch_margin_left: i32) {
        self.patch_margin_left = patch_margin_left;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_patch_margin_left(&self) -> i32 {
        self.patch_margin_left
    }

    #[func]
    fn set_patch_margin_top(&mut self, patch_margin_top: i32) {
        self.patch_margin_top = patch_margin_top;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_patch_margin_top(&self) -> i32 {
        self.patch_margin_top
    }

    #[func]
    fn set_patch_margin_right(&mut self, patch_margin_right: i32) {
        self.patch_margin_right = patch_margin_right;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_patch_margin_right(&self) -> i32 {
        self.patch_margin_right
    }

    #[func]
    fn set_patch_margin_bottom(&mut self, patch_margin_bottom: i32) {
        self.patch_margin_bottom = patch_margin_bottom;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_patch_margin_bottom(&self) -> i32 {
        self.patch_margin_bottom
    }

    #[func]
    fn get_h_axis_stretch_mode(&self) -> AxisStretchMode {
        self.axis_stretch_horizontal
    }

    #[func]
    fn set_h_axis_stretch_mode(&mut self, mode: AxisStretchMode) {
        self.axis_stretch_horizontal = mode;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn get_v_axis_stretch_mode(&self) -> AxisStretchMode {
        self.axis_stretch_vertical
    }

    #[func]
    fn set_v_axis_stretch_mode(&mut self, mode: AxisStretchMode) {
        self.axis_stretch_vertical = mode;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn is_centered(&self) -> bool {
        self.centered
    }

    #[func]
    fn set_centered(&mut self, centered: bool) {
        self.centered = centered;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn get_offset(&self) -> Vector2 {
        self.offset
    }

    #[func]
    fn set_offset(&mut self, offset: Vector2) {
        self.offset = offset;
        self.emit_item_rect_changed_and_redraw();
    }

    #[func]
    fn is_flipped_h(&self) -> bool {
        self.flip_h
    }

    #[func]
    fn set_flip_h(&mut self, flip_h: bool) {
        self.flip_h = flip_h;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn is_flipped_v(&self) -> bool {
        self.flip_v
    }

    #[func]
    fn set_flip_v(&mut self, flip_v: bool) {
        self.flip_v = flip_v;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn get_hframes(&self) -> i32 {
        self.hframes
    }

    #[func]
    fn set_hframes(&mut self, frames: i32) {
        self.hframes = frames.max(1);
        self.set_frame(self.frame);
    }

    #[func]
    fn get_vframes(&self) -> i32 {
        self.vframes
    }

    #[func]
    fn set_vframes(&mut self, frames: i32) {
        self.vframes = frames.max(1);
        self.set_frame(self.frame);
    }

    #[func]
    fn get_frame(&self) -> i32 {
        self.frame
    }

    #[func]
    fn set_frame(&mut self, frame: i32) {
        let max_frame = self.hframes.saturating_mul(self.vframes).saturating_sub(1);
        self.frame = frame.clamp(0, max_frame);

        let as_vec = Vector2i::new(self.frame % self.hframes, self.frame / self.hframes);
        if self.frame_coords != as_vec {
            self.set_frame_coords(as_vec);
            self.base_mut().notify_property_list_changed();
            return;
        }

        self.emit_item_rect_changed_and_redraw();
        self.signals().frame_changed().emit();
    }

    #[func]
    fn get_frame_coords(&self) -> Vector2i {
        self.frame_coords
    }

    #[func]
    fn set_frame_coords(&mut self, coords: Vector2i) {
        self.frame_coords = clamp_vec2i(
            coords,
            Vector2i::ZERO,
            Vector2i::new(self.hframes - 1, self.vframes - 1),
        );

        let as_int = self.frame_coords.x + self.frame_coords.y * self.hframes;
        if self.frame != as_int {
            self.set_frame(as_int);
            self.base_mut().notify_property_list_changed();
            return;
        }

        self.emit_item_rect_changed_and_redraw();
        self.signals().frame_changed().emit();
    }

    fn emit_item_rect_changed_and_redraw(&mut self) {
        self.signals().item_rect_changed().emit();
        self.base_mut().queue_redraw();
    }
}

impl AxisStretchMode {
    fn to_rendering_server(self) -> NinePatchAxisMode {
        match self {
            Self::AxisStretchModeStretch => NinePatchAxisMode::STRETCH,
            Self::AxisStretchModeTile => NinePatchAxisMode::TILE,
            Self::AxisStretchModeTileFit => NinePatchAxisMode::TILE_FIT,
        }
    }
}

fn clamp_vec2(value: Vector2, min: Vector2, max: Vector2) -> Vector2 {
    Vector2::new(value.x.clamp(min.x, max.x), value.y.clamp(min.y, max.y))
}

fn clamp_vec2i(value: Vector2i, min: Vector2i, max: Vector2i) -> Vector2i {
    Vector2i::new(value.x.clamp(min.x, max.x), value.y.clamp(min.y, max.y))
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[godot(via = i64)]
pub enum AxisStretchMode {
    #[default]
    AxisStretchModeStretch = 0,
    AxisStretchModeTile = 1,
    AxisStretchModeTileFit = 2,
}
