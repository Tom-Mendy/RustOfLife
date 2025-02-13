use std::i32;

use rust_of_life::game::{Game, GameStatus};
use rust_of_life::sdl_lib::{
    draw_game, generate_texture, get_target_for_texture, handle_event, init_canvas, init_font,
    init_ttf_context, BLACK,
};

use rust_of_life::utils::{game_of_life, get_grid_point_list, get_rect_list};

use sdl2::rect::{FPoint, FRect};
use sdl2::render::Texture;

fn main() -> Result<(), String> {
    let mut game_info: Game = Game::new();

    let (sdl_context, mut canvas) = init_canvas(
        &game_info.get_name(),
        game_info.get_window_min_length() as u32,
        game_info.get_window_min_length() as u32,
        BLACK,
    )?;
    //println!("game_info is {game_info:?}");

    let mut event_pump = sdl_context.event_pump()?;

    let grid_point_list = get_grid_point_list(
        game_info.get_size_grid(),
        game_info.get_unit_grid(),
        game_info.get_window_min_length(),
        game_info.get_window_min_length(),
    );
    // Convert Vec<Point> into a borrowed slice
    let points_slice: &[FPoint] = grid_point_list.as_slice();
    // The following demonstrates a type that implements Into<&[Point]>
    let mut tmp_vec = points_slice.to_vec();
    let mut borrowed_slice: &[FPoint] = &tmp_vec[..];

    // Initialize TTF context
    let ttf_context = init_ttf_context();

    // Load font
    let font = init_font("./assets/Roboto-Medium.ttf", 40, &ttf_context)?;

    // Render the text to a surface, then create a texture
    let texture_creator = canvas.texture_creator();
    let mut texture_iteration = generate_texture(&font, "iteration: 0", BLACK, &texture_creator)?;
    let mut texture_iteration_per_second: Texture<'_> =
        generate_texture(&font, "iteration / s: 0", BLACK, &texture_creator)?;

    // Query the texture for width and height
    let mut target_iteration = get_target_for_texture(&texture_iteration, 0, 0);
    let mut target_population: FRect;
    let mut target_iteration_per_second: FRect =
        get_target_for_texture(&texture_iteration_per_second, 0, 200);

    // Draw the texture to the canvas
    // let mut list_color_save: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut list_color: Vec<Vec<bool>> =
        vec![vec![false; game_info.get_size_grid() as usize]; game_info.get_size_grid() as usize];

    canvas.set_draw_color(BLACK);

    let mut start_text_point = [0, 0];

    while game_info.get_game_state() != GameStatus::Exit {
        handle_event(&mut event_pump, &mut list_color, &mut game_info);

        // if the window is resized, update the grid
        if canvas.window().size().0 != game_info.get_window_width()
            || canvas.window().size().1 != game_info.get_window_height()
        {
            game_info.set_window_width(canvas.window().size().0);
            game_info.set_window_height(canvas.window().size().1);

            let tmp_grid_point_list = get_grid_point_list(
                game_info.get_size_grid(),
                game_info.get_unit_grid(),
                game_info.get_window_min_length(),
                game_info.get_window_min_length(),
            );
            // Convert Vec<Point> into a borrowed slice
            let tmp_slice: &[FPoint] = tmp_grid_point_list.as_slice();
            // The following demonstrates a type that implements Into<&[Point]>
            tmp_vec = tmp_slice.to_vec();
            borrowed_slice = &tmp_vec[..];
            start_text_point = [0, 0];

            if game_info.get_window_height() as f32
                - game_info.get_size_grid() as f32 * game_info.get_unit_grid()
                > 100.0
            {
                start_text_point[1] =
                    (game_info.get_size_grid() as f32 * game_info.get_unit_grid()) as u32;
            } else if game_info.get_window_width() as f32
                - game_info.get_size_grid() as f32 * game_info.get_unit_grid()
                > 100.0
            {
                start_text_point[0] =
                    (game_info.get_size_grid() as f32 * game_info.get_unit_grid()) as u32;
            }

            texture_iteration = generate_texture(
                &font,
                &("iteration: ".to_string() + &game_info.get_iteration().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_iteration = get_target_for_texture(
                &texture_iteration,
                start_text_point[0] as i32,
                start_text_point[1] as i32,
            );
            texture_iteration_per_second = generate_texture(
                &font,
                &("iteration / s: ".to_string()
                    + &game_info.get_iteration_per_second().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_iteration_per_second = get_target_for_texture(
                &texture_iteration_per_second,
                start_text_point[0] as i32,
                start_text_point[1] as i32 + 200,
            );
        }

        if game_info.get_game_state() != GameStatus::Pause
        // && game_info.get_iteration_per_second()
        //     < game_info.get_max_iteration_per_second() as f64
        {
            //let ticks = timer.ticks() as i32;

            // save the grid
            // list_color_save.push(list_color.clone());
            // update the grid
            list_color = game_of_life(list_color);

            texture_iteration = generate_texture(
                &font,
                &("iteration: ".to_string() + &game_info.get_iteration().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_iteration = get_target_for_texture(
                &texture_iteration,
                start_text_point[0] as i32,
                start_text_point[1] as i32,
            );
            texture_iteration_per_second = generate_texture(
                &font,
                &("iteration / s: ".to_string()
                    + &game_info.get_iteration_per_second().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_iteration_per_second = get_target_for_texture(
                &texture_iteration_per_second,
                start_text_point[0] as i32,
                start_text_point[1] as i32 + 200,
            );
            game_info.set_iteration(game_info.get_iteration() + 1);
        }

        // display the grid
        canvas.clear();
        if game_info.get_game_state() != GameStatus::Exit {
            let cell_rects = get_rect_list(&list_color, game_info.get_unit_grid());
            let texture_population = generate_texture(
                &font,
                &("population: ".to_string() + &cell_rects.len().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_population = get_target_for_texture(
                &texture_population,
                start_text_point[0] as i32,
                start_text_point[1] as i32 + 100,
            );
            draw_game(
                &mut canvas,
                &borrowed_slice,
                &cell_rects,
                &texture_iteration,
                &texture_population,
                &texture_iteration_per_second,
                target_iteration,
                target_population,
                target_iteration_per_second,
            );
            canvas.present();
        }
    }

    Ok(())
}
