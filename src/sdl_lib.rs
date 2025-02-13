// use chrono::Local;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

pub mod sdl_lib {
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

    use crate::game::game::{Game, GameStatus};

    pub const WHITE: Color = Color::RGB(255, 255, 255);
    pub const BLACK: Color = Color::RGB(0, 0, 0);

    pub fn handle_event(
        event_pump: &mut sdl2::EventPump,
        list_color: &mut Vec<Vec<bool>>,
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
                    // if the left mouse button is pressed
                    if mousestate.left() {
                        let cell_x = x / game_info.get_unit_grid() as i32;
                        let cell_y = y / game_info.get_unit_grid() as i32;

                        if cell_x >= 0
                            && cell_x < game_info.get_window_width() as i32
                            && cell_y >= 0
                            && cell_y < game_info.get_window_height() as i32
                        {
                            list_color[cell_y as usize][cell_x as usize] = true;
                        }
                    }
                    // right click to remove the cell
                    if mousestate.right() {
                        let cell_x = x / game_info.get_unit_grid() as i32;
                        let cell_y = y / game_info.get_unit_grid() as i32;

                        if cell_x >= 0
                            && cell_x < game_info.get_window_width() as i32
                            && cell_y >= 0
                            && cell_y < game_info.get_window_height() as i32
                        {
                            list_color[cell_y as usize][cell_x as usize] = false;
                        }
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let cell_x = x / game_info.get_unit_grid() as i32;
                    let cell_y = y / game_info.get_unit_grid() as i32;

                    if cell_x >= 0
                        && cell_x < game_info.get_size_grid() as i32
                        && cell_y >= 0
                        && cell_y < game_info.get_size_grid() as i32
                    {
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
        texture_iteration: &Texture,
        texture_population: &Texture,
        texture_iteration_per_second: &Texture,
        target_iteration: FRect,
        target_population: FRect,
        target_iteration_per_second: FRect,
    ) {
        canvas.set_draw_color(BLACK);
        if let Err(e) = canvas.draw_flines(list_lines) {
            eprintln!("Error drawing lines: {}", e);
        }
        if let Err(e) = canvas.fill_frects(&cell_rects) {
            eprintln!("Error filling rectangles: {}", e);
        }
        canvas.set_draw_color(WHITE);

        // Draw number of iteration
        if let Err(e) = canvas.copy_f(&texture_iteration, None, Some(target_iteration)) {
            eprintln!("Error copying texture_iteration: {}", e);
        }
        if let Err(e) = canvas.copy_f(&texture_population, None, Some(target_population)) {
            eprintln!("Error copying texture_population: {}", e);
        }
        if let Err(e) = canvas.copy_f(
            &texture_iteration_per_second,
            None,
            Some(target_iteration_per_second),
        ) {
            eprintln!("Error copying texture_iteration_per_second: {}", e);
        }
    }

    //fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32) -> Result<(), String> {
    //    let mut x = radius;
    //    let mut y = 0;
    //
    //    let mut re = x * x + y * y - radius * radius;
    //    while x >= y {
    //        canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
    //        canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;
    //
    //        canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
    //        canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;
    //
    //        canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
    //        canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;
    //
    //        canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
    //        canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;
    //
    //        if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
    //            re += 1 - 2 * x;
    //            x -= 1;
    //        }
    //        re += 2 * y + 1;
    //        y += 1;
    //    }
    //
    //    Ok(())
    //}
}
