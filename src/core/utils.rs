use macroquad::prelude::*;

pub fn draw_text_center(message: &str, font_size: u16, pos_y: f32)
{
    let measure = measure_text(message, None, font_size, 1.0);
    draw_text(
        message,
        (screen_width() - measure.width) / 2.0,
        pos_y - 10.0 - measure.height,
        font_size as f32,
        BLACK,
    );
}