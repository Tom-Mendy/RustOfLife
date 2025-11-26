use sdl2::pixels::Color;
use sdl2::rect::{FPoint, FRect};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::{self, Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

/// Structure holding a texture and its target rectangle.
pub struct TextureWithRect<'a> {
    pub texture: &'a Texture<'a>,
    pub target: FRect,
}

/// Generates a texture from rendered text using the provided font and color.
pub fn generate_texture<'a>(
    font: &Font,
    text: &str,
    color: Color,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let surface: Surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture: Texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    Ok(texture)
}

/// Initializes the SDL2 TTF context.
/// Returns a Result containing the context or an error message.
pub fn init_ttf_context() -> Result<Sdl2TtfContext, String> {
    ttf::init().map_err(|e| format!("Failed to initialize TTF context: {}", e))
}

/// Loads a font from the given path and size using the provided TTF context.
pub fn init_font<'a>(
    font_path: &str,
    font_size: u16,
    ttf_context: &'a Sdl2TtfContext,
) -> Result<Font<'a, 'a>, String> {
    let font = ttf_context
        .load_font(font_path, font_size)
        .map_err(|e| e.to_string())?;
    Ok(font)
}

/// Initializes the SDL2 canvas and window with the given parameters.
/// Returns the SDL context and canvas, or an error message.
pub fn init_canvas(
    title: &str,
    width: u32,
    height: u32,
    draw_color: Color,
) -> Result<(sdl2::Sdl, Canvas<Window>), String> {
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
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

pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);

pub fn get_target_for_texture(
    texture: &Texture,
    position_width: i32,
    position_height: i32,
) -> FRect {
    let TextureQuery {
        width: texture_width,
        height: texture_height,
        ..
    } = texture.query();
    FRect::new(
        position_width as f32,
        position_height as f32,
        texture_width as f32,
        texture_height as f32,
    )
}

pub fn draw_game(
    canvas: &mut Canvas<Window>,
    list_lines: &[FPoint],
    cell_rects: &[FRect],
    iteration: TextureWithRect,
    population: TextureWithRect,
    iteration_per_second: TextureWithRect,
) {
    canvas.set_draw_color(BLACK);
    if let Err(e) = canvas.draw_flines(list_lines) {
        eprintln!("Error drawing lines: {}", e);
    }
    if let Err(e) = canvas.fill_frects(cell_rects) {
        eprintln!("Error filling rectangles: {}", e);
    }
    canvas.set_draw_color(WHITE);
    if let Err(e) = canvas.copy_f(iteration.texture, None, Some(iteration.target)) {
        eprintln!("Error copying texture_iteration: {}", e);
    }
    if let Err(e) = canvas.copy_f(population.texture, None, Some(population.target)) {
        eprintln!("Error copying texture_population: {}", e);
    }
    if let Err(e) = canvas.copy_f(
        iteration_per_second.texture,
        None,
        Some(iteration_per_second.target),
    ) {
        eprintln!("Error copying texture_iteration_per_second: {}", e);
    }
}
