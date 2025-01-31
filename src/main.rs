use std::cmp::min;
use std::{i32, iter};

mod sdl_lib;
use crate::sdl_lib::sdl_lib::{
    draw_game, generate_texture, get_target_for_texture, handle_even, init_canvas, init_font,
    init_ttf_context, BLACK,
};
mod game;
use crate::game::game::{Game, GameStatus};

use sdl2::rect::{FPoint, FRect};
use sdl2::render::Texture;
use std::sync::{Arc, Mutex};
use std::thread;

fn get_grid_point_list(
    size_grid: u32,
    unit_grid: f32,
    window_height: u32,
    window_width: u32,
) -> Vec<FPoint> {
    let mut grid_point_list = Vec::new();
    for i in 0..size_grid {
        if (i % 2) == 0 {
            grid_point_list.push(FPoint::new(unit_grid * i as f32, 0.0));
            grid_point_list.push(FPoint::new(unit_grid * i as f32, window_height as f32));
        } else {
            grid_point_list.push(FPoint::new(unit_grid * i as f32, window_height as f32));
            grid_point_list.push(FPoint::new(unit_grid * i as f32, 0.0));
        }
    }
    grid_point_list.push(FPoint::new(window_width as f32, 0.0));
    grid_point_list.push(FPoint::new(0.0, 0.0));
    for i in 0..size_grid {
        if (i % 2) == 0 {
            grid_point_list.push(FPoint::new(0.0, unit_grid * i as f32));
            grid_point_list.push(FPoint::new(window_width as f32, unit_grid * i as f32));
        } else {
            grid_point_list.push(FPoint::new(window_width as f32, unit_grid * i as f32));
            grid_point_list.push(FPoint::new(0.0, unit_grid * i as f32));
        }
    }
    grid_point_list
}

//fn is_rect_in_list(rect: &Rect, list_rect: &Vec<Rect>) -> bool {
//    for r in list_rect {
//        if r.x() == rect.x() && r.y() == rect.y() {
//            return true;
//        }
//    }
//    false
//}

fn get_number_black_around_cell(list: &Vec<Vec<bool>>, x: i32, y: i32) -> i32 {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < list.len() as i32 && ny >= 0 && ny < list[0].len() as i32 {
            if list[nx as usize][ny as usize] {
                count += 1;
            }
        }
    }

    count
}

fn game_of_life(list: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let new_list = Arc::new(Mutex::new(vec![vec![false; list[0].len()]; list.len()]));

    let mut handles = vec![];

    for (i, row) in list.clone().into_iter().enumerate() {
        let new_list = Arc::clone(&new_list);
        let list_clone = list.clone();
        let handle = thread::spawn(move || {
            let list = list_clone;
            let mut new_row = vec![false; row.len()];
            for (j, &cell) in row.iter().enumerate() {
                let count_black_neighbor = get_number_black_around_cell(&list, i as i32, j as i32);
                new_row[j] = match cell {
                    true => (2..=3).contains(&count_black_neighbor),
                    false => count_black_neighbor == 3,
                };
            }
            new_list.lock().unwrap()[i] = new_row;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(new_list).unwrap().into_inner().unwrap()
}

// fn get_population(list: &Vec<Vec<bool>>) -> i32 {
//     let mut count = 0;
//     for i in 0..list.len() {
//         for j in 0..list[i].len() {
//             if list[i][j] {
//                 count += 1;
//             }
//         }
//     }
//     count
// }

fn get_rect_list(list: &Vec<Vec<bool>>, unit_grid: f32) -> Vec<FRect> {
    let mut list_rect: Vec<FRect> = Vec::new();
    for i in 0..list.len() {
        for j in 0..list[i].len() {
            if list[i][j] {
                list_rect.push(FRect::new(
                    j as f32 * unit_grid,
                    i as f32 * unit_grid,
                    unit_grid,
                    unit_grid,
                ));
            }
        }
    }
    list_rect
}

fn main() -> Result<(), String> {
    let mut game_info: Game = Game::new();

    let (sdl_context, mut canvas) = init_canvas(
        &game_info.get_name(),
        game_info.get_window_width() as u32,
        game_info.get_window_height() as u32,
        BLACK,
    )?;
    //println!("game_info is {game_info:?}");

    let mut event_pump = sdl_context.event_pump()?;

    let grid_point_list = get_grid_point_list(
        game_info.get_size_grid(),
        game_info.get_unit_grid(),
        game_info.get_window_height(),
        game_info.get_window_width(),
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

    let mut window_min_length = min(canvas.window().size().0, canvas.window().size().1);

    while game_info.get_game_state() != GameStatus::Exit {
        handle_even(&mut event_pump, &mut list_color, &mut game_info);

        if canvas.window().size().0 != window_min_length
            || canvas.window().size().1 != window_min_length
        {
            window_min_length = min(canvas.window().size().0, canvas.window().size().1);
            game_info.set_window_height(window_min_length);
            game_info.set_window_width(window_min_length);

            let tmp_grid_point_list = get_grid_point_list(
                game_info.get_size_grid(),
                game_info.get_unit_grid(),
                game_info.get_window_height(),
                game_info.get_window_width(),
            );
            // Convert Vec<Point> into a borrowed slice
            let tmp_slice: &[FPoint] = tmp_grid_point_list.as_slice();
            // The following demonstrates a type that implements Into<&[Point]>
            tmp_vec = tmp_slice.to_vec();
            borrowed_slice = &tmp_vec[..];
        }

        if game_info.get_game_state() != GameStatus::Pause
            && game_info.get_iteration_per_second()
                < game_info.get_max_iteration_per_second() as f64
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
            target_iteration = get_target_for_texture(&texture_iteration, 0, 0);
            texture_iteration_per_second = generate_texture(
                &font,
                &("iteration / s: ".to_string()
                    + &game_info.get_iteration_per_second().to_string()),
                BLACK,
                &texture_creator,
            )?;
            target_iteration_per_second =
                get_target_for_texture(&texture_iteration_per_second, 0, 200);
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
            target_population = get_target_for_texture(&texture_population, 0, 100);
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
