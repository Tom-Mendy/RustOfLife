#[cfg(test)]
mod tests {

    use rust_of_life::game::Game;
    use rust_of_life::sdl_lib::handle_event;

    #[test]
    fn test_handle_event_mouse_motion() {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut list_color = vec![vec![false; 10]; 10];
        let mut game_info = Game::new();
        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::MouseMotion {
                timestamp: 0,
                window_id: 0,
                which: 0,
                mousestate: sdl2::mouse::MouseState::from_sdl_state(1),
                x: 0,
                y: 0,
                xrel: 100,
                yrel: 0,
            })
            .unwrap();
        handle_event(&mut event_pump, &mut list_color, &mut game_info);

        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::MouseMotion {
                timestamp: 0,
                window_id: 0,
                which: 0,
                mousestate: sdl2::mouse::MouseState::from_sdl_state(2),
                x: 0,
                y: 0,
                xrel: 100,
                yrel: 0,
            })
            .unwrap();
        handle_event(&mut event_pump, &mut list_color, &mut game_info);

        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::MouseMotion {
                timestamp: 0,
                window_id: 0,
                which: 0,
                mousestate: sdl2::mouse::MouseState::from_sdl_state(3),
                x: 0,
                y: 0,
                xrel: 100,
                yrel: 0,
            })
            .unwrap();
        handle_event(&mut event_pump, &mut list_color, &mut game_info);

        sdl_context
            .event()
            .unwrap()
            .push_event(sdl2::event::Event::MouseMotion {
                timestamp: 0,
                window_id: 0,
                which: 0,
                mousestate: sdl2::mouse::MouseState::from_sdl_state(4),
                x: 0,
                y: 0,
                xrel: 100,
                yrel: 0,
            })
            .unwrap();

        handle_event(&mut event_pump, &mut list_color, &mut game_info);
    }
}
