use crate::game::{Game, GameStatus};
use chrono::Local;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

/// Handles SDL2 events and updates game state accordingly.
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
