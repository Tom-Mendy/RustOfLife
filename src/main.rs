use std::ops::Index;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::{self, Font};
use sdl2::video::Window;

use rand::Rng;

fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32) -> Result<(), String> {
    let mut x = radius;
    let mut y = 0;

    let mut re = x * x + y * y - radius * radius;
    while x >= y {
        canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;

        canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;

        if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
            re += 1 - 2 * x;
            x -= 1;
        }
        re += 2 * y + 1;
        y += 1;
    }

    Ok(())
}

fn get_grid_point_list(
    size_grid: i32,
    unit_grid: i32,
    window_height: i32,
    window_width: i32,
) -> Vec<Point> {
    let mut grid_point_linst = Vec::new();
    for i in 0..size_grid {
        if (i % 2) == 0 {
            grid_point_linst.push(Point::new(unit_grid * (i as i32), 0));
            grid_point_linst.push(Point::new(unit_grid * (i as i32), window_height));
        } else {
            grid_point_linst.push(Point::new(unit_grid * (i as i32), window_height));
            grid_point_linst.push(Point::new(unit_grid * (i as i32), 0));
        }
    }
    grid_point_linst.push(Point::new(window_width, 0));
    grid_point_linst.push(Point::new(0, 0));
    for i in 0..size_grid {
        if (i % 2) == 0 {
            grid_point_linst.push(Point::new(0, unit_grid * (i as i32)));
            grid_point_linst.push(Point::new(window_width, unit_grid * (i as i32)));
        } else {
            grid_point_linst.push(Point::new(window_width, unit_grid * (i as i32)));
            grid_point_linst.push(Point::new(0, unit_grid * (i as i32)));
        }
    }
    return grid_point_linst;
}

fn is_rect_in_list(rect: &Rect, list_rect: &Vec<Rect>) -> bool {
    for r in list_rect {
        if r.x() == rect.x() && r.y() == rect.y() {
            return true;
        }
    }
    return false;
}

fn handle_even(event_pump: &mut sdl2::EventPump, list_rect: &mut Vec<Rect>, game_info: &mut Game) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                (*game_info).game_state = GameStatus::Exit;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => match (*game_info).game_state {
                GameStatus::Pause => {
                    (*game_info).game_state = GameStatus::Running;
                }
                GameStatus::Running => {
                    (*game_info).game_state = GameStatus::Pause;
                }
                _ => {}
            },
            // drag and slide the grid cell
            //Event::MouseMotion {
            //    timestamp,
            //    window_id,
            //    which,
            //    mousestate,
            //    x,
            //    y,
            //    xrel,
            //    yrel,
            //} => {
            //    println!("MouseMotion: x={}, y={}", x, y);
            //}
            Event::MouseButtonDown { x, y, .. } => {
                let cell_x = (x / game_info.unit_grid) * game_info.unit_grid;
                let cell_y = (y / game_info.unit_grid) * game_info.unit_grid;
                if is_rect_in_list(
                    &Rect::new(
                        cell_x,
                        cell_y,
                        game_info.unit_grid as u32,
                        game_info.unit_grid as u32,
                    ),
                    list_rect,
                ) {
                    list_rect.retain(|r| r.x() != cell_x || r.y() != cell_y);
                } else {
                    list_rect.push(Rect::new(
                        cell_x,
                        cell_y,
                        game_info.unit_grid as u32,
                        game_info.unit_grid as u32,
                    ));
                }
            }
            _ => {}
        }
    }
}

#[derive(PartialEq)]
enum GameStatus {
    Exit,
    Pause,
    Running,
}

struct Game {
    game_state: GameStatus,
    size_grid: i32,
    window_height: i32,
    window_width: i32,
    unit_grid: i32,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut game_info: Game = Game {
        game_state: GameStatus::Running,
        size_grid: 100,
        window_height: 1000,
        window_width: 1000,
        unit_grid: 0, // will be calculated later
    };
    game_info.unit_grid = (game_info.window_width / game_info.size_grid) as i32;

    let window = video_subsystem
        .window(
            "Rust Of Life",
            game_info.window_width as u32,
            game_info.window_height as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    //let texture_creator = canvas.texture_creator();
    //let mut tex = texture_creator
    //    .create_texture_target(None, 400, 400)
    //    .map_err(|_| String::from("Unable to create texture."))?;

    //let timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

    let grid_point_linst = get_grid_point_list(
        game_info.size_grid,
        game_info.unit_grid,
        game_info.window_height,
        game_info.window_width,
    );
    // Convert Vec<Point> into a borrowed slice
    let points_slice: &[Point] = grid_point_linst.as_slice();

    // The following demonstrates a type that implements Into<&[Point]>
    let borrowed_slice: &[Point] = &points_slice.iter().map(|&p| p).collect::<Vec<Point>>()[..];

    // Initialize the TTF context
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    // Load the font using `from_file`
    //let font = Font::from_file(&ttf_context, "Roboto-Medium.ttf", 128).map_err(|e| e.to_string());

    let mut list_rect: Vec<Rect> = Vec::new();
    while game_info.game_state != GameStatus::Exit {
        handle_even(&mut event_pump, &mut list_rect, &mut game_info);

        if game_info.game_state != GameStatus::Pause {
            //let ticks = timer.ticks() as i32;

            // update the grid
            let new_list: Vec<Rect> = list_rect
                .clone()
                .into_iter()
                .map(|mut r| {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(0..=game_info.size_grid);
                    let y = rng.gen_range(0..=game_info.size_grid);
                    r.x = x * game_info.unit_grid;
                    r.y = y * game_info.unit_grid;
                    return r;
                })
                .collect::<Vec<Rect>>();
            list_rect = new_list;
        }
        // display the grid
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.draw_lines(borrowed_slice)?;
        canvas.fill_rects(&list_rect)?;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        //font.render("Hello Rust!", sdl2::pixels::Color::RGB(0, 0, 0))
        //    .map_err(|e| e.to_string())?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
