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

fn handle_even(event_pump: &mut sdl2::EventPump, running: &mut bool, pause: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *running = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                *pause = !*pause;
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let size_grid = 100;
    let window_height = 1000;
    let window_width = 1000;
    let window = video_subsystem
        .window("Rust Of Life", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let unit_grid: i32 = (window_width / size_grid) as i32;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    let texture_creator = canvas.texture_creator();
    let mut tex = texture_creator
        .create_texture_target(None, 400, 400)
        .map_err(|_| String::from("Unable to create texture."))?;

    let timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut running = true;

    let grid_point_linst = get_grid_point_list(
        size_grid as i32,
        unit_grid,
        window_height as i32,
        window_width as i32,
    );
    // Convert Vec<Point> into a borrowed slice
    let points_slice: &[Point] = grid_point_linst.as_slice();

    // The following demonstrates a type that implements Into<&[Point]>
    let borrowed_slice: &[Point] = &points_slice.iter().map(|&p| p).collect::<Vec<Point>>()[..];

    // Initialize the TTF context
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    let mut pause: bool = false;

    // Load the font using `from_file`
    //let font = Font::from_file(&ttf_context, "Roboto-Medium.ttf", 128).map_err(|e| e.to_string());
    while running {
        handle_even(&mut event_pump, &mut running, &mut pause);

        if !pause {
            let ticks = timer.ticks() as i32;

            canvas.clear();
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.draw_lines(borrowed_slice)?;
            let list_rect = [
                Rect::new(
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    unit_grid as u32,
                    unit_grid as u32,
                ),
                Rect::new(
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    unit_grid as u32,
                    unit_grid as u32,
                ),
                Rect::new(
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    (rand::thread_rng().gen_range(0..=size_grid - 1) as i32) * unit_grid,
                    unit_grid as u32,
                    unit_grid as u32,
                ),
            ];
            canvas.fill_rects(&list_rect)?;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            //font.render("Hello Rust!", sdl2::pixels::Color::RGB(0, 0, 0))
            //    .map_err(|e| e.to_string())?;
            canvas.present();
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
