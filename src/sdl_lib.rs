// use chrono::Local;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::rect::{Point, Rect};

pub mod sdl_lib {
    use sdl2::render::Canvas;
    use sdl2::pixels::Color;
    use sdl2::video::Window;
    use sdl2::surface::Surface;
    use sdl2::render::{Texture, TextureCreator};
    use sdl2::ttf::{self, Font, Sdl2TtfContext};
    use sdl2::video::WindowContext;

    pub fn generate_texture<'a>(
        font: &Font,
        text: &str,
        color: Color,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Texture<'a>, String> {
        // Render the text to a surface
        let surface: Surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())?;

        // Create a texture from the surface
        let texture: Texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        Ok(texture)
    }

    pub fn init_ttf_context() -> Sdl2TtfContext {
        ttf::init().map_err(|e| e.to_string()).unwrap()
    }

    pub fn init_font<'a>(
        font_path: &str,
        font_size: u16,
        ttf_context: &'a Sdl2TtfContext,
    ) -> Result<Font<'a, 'a>, String> {
        // Load the font
        let font = ttf_context
            .load_font(font_path, font_size)
            .map_err(|e| e.to_string())?;

        Ok(font)
    }

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
