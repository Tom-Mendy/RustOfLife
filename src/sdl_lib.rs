// use chrono::Local;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::rect::{Point, Rect};
// use sdl2::render::{Texture, TextureCreator, TextureQuery};
// use sdl2::surface::Surface;
// use sdl2::ttf::{self, Font, Sdl2TtfContext};
// use sdl2::video::WindowContext;

pub mod sdl_lib {
    use sdl2::render::Canvas;
    use sdl2::pixels::Color;
    use sdl2::video::Window;

    pub fn init_canvas(
        title: &str,
        width: u32,
        height: u32,
        draw_color: Color,
    ) -> Result<(sdl2::Sdl, Canvas<Window>), String> {
        // Initialize SDL2
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        // Create a window
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        // Initialize the canvas
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(draw_color);
        canvas.clear();
        canvas.present();

        Ok((sdl_context, canvas))
    }
}
