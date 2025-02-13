#[cfg(test)]
mod tests {
    use rust_of_life::sdl_lib;
    use sdl2::pixels::Color;

    #[test]
    fn test_init_canvas() {
        let result = sdl_lib::init_canvas("Test", 800, 600, Color::RGB(0, 0, 0));
        assert!(result.is_ok());
    }
}
