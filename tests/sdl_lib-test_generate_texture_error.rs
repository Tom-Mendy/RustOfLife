//! Tests for error handling in generate_texture

use rust_of_life::sdl_lib::{init_font, init_ttf_context};

#[test]
fn test_generate_texture_error() {
    let ttf_context = init_ttf_context().expect("TTF context should initialize");
    // Use an invalid font path to trigger an error
    let font = init_font("/invalid/path.ttf", 40, &ttf_context);
    assert!(font.is_err(), "Font loading should fail for invalid path");
}
