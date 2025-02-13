#[cfg(test)]
mod tests {

    use rust_of_life::game::Game;
    use rust_of_life::sdl_lib::handle_event;
    use sdl2::keyboard::Keycode;

    #[test]
    fn test_handle_event_key_space() {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut list_color = vec![vec![false; 10]; 10];
        let mut game_info = Game::new();
        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::KeyDown {
                timestamp: 0,
                window_id: 0,
                keycode: Some(Keycode::Space),
                scancode: None,
                keymod: sdl2::keyboard::Mod::NOMOD,
                repeat: false,
            })
            .unwrap();

        handle_event(&mut event_pump, &mut list_color, &mut game_info);
        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::KeyDown {
                timestamp: 0,
                window_id: 0,
                keycode: Some(Keycode::Space),
                scancode: None,
                keymod: sdl2::keyboard::Mod::NOMOD,
                repeat: false,
            })
            .unwrap();
        handle_event(&mut event_pump, &mut list_color, &mut game_info);
    }
}
