use sdl2::pixels::Color;
pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);
pub mod events;
pub mod render;

pub use events::handle_event;
pub use render::{
    draw_game, generate_texture, get_target_for_texture, init_canvas, init_font, init_ttf_context,
    TextureWithRect,
};
