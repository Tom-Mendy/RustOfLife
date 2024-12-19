use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Of Life", 1000, 1000)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

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
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;

        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas
            .with_texture_canvas(&mut tex, |the_canvas| {
                draw_circle(the_canvas, Point::new(200, 200), 50);
            })
            .map_err(|_| String::from("Failed to draw on texture"))?;
        canvas.copy(&tex, None, Rect::new(0, 0, 400, 400))?;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
