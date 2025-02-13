use std::i32;

use sdl2::rect::{FPoint, FRect};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn get_grid_point_list(
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

pub fn game_of_life(list: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
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

pub fn get_rect_list(list: &Vec<Vec<bool>>, unit_grid: f32) -> Vec<FRect> {
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
