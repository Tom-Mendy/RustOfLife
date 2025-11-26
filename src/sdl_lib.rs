use chrono::Local;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::FPoint;
use sdl2::rect::FRect;
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::{self, Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::game::{Game, GameStatus};

pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);

pub struct TextureWithRect<'a> {
    pub texture: &'a Texture<'a>,
    pub target: FRect,
}

fn calculate_cell_position(x: i32, y: i32, game_info: &Game) -> (i32, i32) {
    let cell_x = (x as f32 / game_info.get_unit_grid()) as i32;
    let cell_y = (y as f32 / game_info.get_unit_grid()) as i32;

    (cell_x, cell_y)
}

fn check_cell_in_map(cell_x: i32, cell_y: i32, game_info: &Game) -> bool {
    cell_x >= 0
        && cell_x < game_info.get_size_grid() as i32
        && cell_y >= 0
        && cell_y < game_info.get_size_grid() as i32
}

pub fn handle_event(
    event_pump: &mut sdl2::EventPump,
    list_color: &mut [Vec<bool>],
    game_info: &mut Game,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                game_info.set_game_state(GameStatus::Exit);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => match game_info.get_game_state() {
                GameStatus::Pause => {
                    game_info.set_game_state(GameStatus::Running);
                    game_info.set_start_time(Local::now());
                    game_info.set_start_time_iteration(game_info.get_iteration());
                }
                GameStatus::Running => {
                    game_info.set_game_state(GameStatus::Pause);
                }
                _ => {}
            },
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                game_info.reset();
                list_color.iter_mut().for_each(|row| {
                    row.iter_mut().for_each(|cell| {
                        *cell = false;
                    });
                });
            }

            // drag and slide the grid cell
            Event::MouseMotion {
                x, y, mousestate, ..
            } => {
                if mousestate.left() {
                    let (cell_x, cell_y) = calculate_cell_position(x, y, game_info);

                    if check_cell_in_map(cell_x, cell_y, game_info) {
                        list_color[cell_y as usize][cell_x as usize] = true;
                    }
                } else if mousestate.right() {
                    let (cell_x, cell_y) = calculate_cell_position(x, y, game_info);

                    if check_cell_in_map(cell_x, cell_y, game_info) {
                        list_color[cell_y as usize][cell_x as usize] = false;
                    }
                }
            }

            Event::MouseButtonDown { x, y, .. } => {
                let (cell_x, cell_y) = calculate_cell_position(x, y, game_info);

                if check_cell_in_map(cell_x, cell_y, game_info) {
                    match list_color[cell_y as usize][cell_x as usize] {
                        true => list_color[cell_y as usize][cell_x as usize] = false,
                        false => list_color[cell_y as usize][cell_x as usize] = true,
                    }
                }
            }
            _ => {}
        }
    }
}

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
    let resr = match ttf::init().map_err(|e| e.to_string()) {
        Ok(context) => context,
        Err(e) => panic!("Failed to initialize TTF context: {}", e),
    };
    resr
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
        .resizable() // Make the window resizable
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

pub fn get_target_for_texture(
    texture: &Texture,
    position_width: i32,
    position_height: i32,
) -> FRect {
    // Query the texture for its width and height
    let TextureQuery {
        width: texture_width,
        height: texture_height,
        ..
    } = texture.query();

    // Calculate the centered position
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

    // Draw number of iteration
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
