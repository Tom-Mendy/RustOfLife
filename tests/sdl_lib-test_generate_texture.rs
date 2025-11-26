#[cfg(test)]
mod tests {
    use rust_of_life::sdl_lib;
    use sdl2::pixels::Color;
    use sdl2::render::TextureCreator;
    use sdl2::video::WindowContext;

    #[test]
    fn test_generate_texture() {
        let ttf_context = sdl_lib::init_ttf_context().unwrap();
        let font = sdl_lib::init_font("./assets/Roboto-Medium.ttf", 128, &ttf_context).unwrap();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("test", 800, 600).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let texture =
            sdl_lib::generate_texture(&font, "Test", Color::RGB(255, 255, 255), &texture_creator);
        assert!(texture.is_ok());
    }
}
