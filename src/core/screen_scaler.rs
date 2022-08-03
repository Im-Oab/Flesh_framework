use macroquad::prelude::*;

/// Use some logic for calculate screen_rect from `Tetra` engine.
pub struct ScreenScaler {
    canvas: RenderTarget,
    camera: Camera2D,
}

impl ScreenScaler {
    pub fn new(inner_width: u32, inner_height: u32) -> Self {
        let canvas = render_target(inner_width, inner_height);
        canvas.texture.set_filter(FilterMode::Nearest);

        let camera = Camera2D {
            zoom: vec2(1. / inner_width as f32 * 2., 1. / inner_height as f32 * 2.),
            target: vec2(inner_width as f32 / 2., inner_height as f32 / 2.),
            render_target: Some(canvas),
            ..Default::default()
        };

        Self {
            canvas: canvas,
            camera: camera,
        }
    }

    pub fn begin(&mut self) {
        set_camera(&self.camera);
    }

    pub fn end(&mut self) {
        set_default_camera();
        clear_background(BLACK);
        let screen_rect = get_screen_rect(
            crate::GAME_WIDTH as u32,
            crate::GAME_HEIGHT as u32,
            screen_width() as u32,
            screen_height() as u32,
        );

        draw_texture_ex(
            self.canvas.texture,
            screen_rect.x,
            screen_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_rect.w, screen_rect.h)),
                ..Default::default()
            },
        );
    }
}

pub fn get_screen_rect(
    inner_width: u32,
    inner_height: u32,
    outer_width: u32,
    outer_height: u32,
) -> Rect {
    let f_inner_width = inner_width as f32;
    let f_inner_height = inner_height as f32;
    let f_outer_width = outer_width as f32;
    let f_outer_height = outer_height as f32;

    let internal_aspect_ratio = f_inner_width / f_inner_height;
    let screen_aspect_ratio = f_outer_width / f_outer_height;

    let scale_factor = if internal_aspect_ratio > screen_aspect_ratio {
        f_outer_width / f_inner_width
    } else {
        f_outer_height / f_inner_height
    };

    let screen_width = (f_inner_width * scale_factor).ceil();
    let screen_height = (f_inner_height * scale_factor).ceil();
    let screen_x = ((f_outer_width - screen_width) / 2.0).ceil();
    let screen_y = ((f_outer_height - screen_height) / 2.0).ceil();

    Rect::new(screen_x, screen_y, screen_width, screen_height)
}
