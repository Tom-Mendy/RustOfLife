#[cfg(test)]
mod tests {
    use rust_of_life::sdl_lib;

    #[test]
    fn test_init_font() {
        let ttf_context = sdl_lib::init_ttf_context();
        let font = sdl_lib::init_font("./assets/Roboto-Medium.ttf", 128, &ttf_context);
        assert!(font.is_ok());
    }
}
