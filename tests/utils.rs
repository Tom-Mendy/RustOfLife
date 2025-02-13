#[cfg(test)]
mod tests {
    use rust_of_life::utils::{game_of_life, get_grid_point_list, get_rect_list};

    use sdl2::rect::{FPoint, FRect};

    #[test]
    fn test_get_grid_point_list() {
        let points = get_grid_point_list(4, 10.0, 100, 100);
        assert_eq!(points.len(), 18);
        assert_eq!(points[0], FPoint::new(0.0, 0.0));
        assert_eq!(points[1], FPoint::new(0.0, 100.0));
        assert_eq!(points[2], FPoint::new(10.0, 100.0));
        assert_eq!(points[3], FPoint::new(10.0, 0.0));
    }

    // #[test]
    // fn test_get_number_black_around_cell() {
    //     let list = vec![
    //         vec![false, true, false],
    //         vec![true, true, false],
    //         vec![false, false, false],
    //     ];
    //     assert_eq!(get_number_black_around_cell(&list, 1, 1), 2);
    //     assert_eq!(get_number_black_around_cell(&list, 0, 0), 2);
    //     assert_eq!(get_number_black_around_cell(&list, 2, 2), 1);
    // }

    #[test]
    fn test_game_of_life() {
        let list = vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, false, false],
        ];
        let expected = vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![false, false, false],
        ];
        assert_eq!(game_of_life(list), expected);
    }

    #[test]
    fn test_get_rect_list() {
        let list = vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, false, false],
        ];
        let rects = get_rect_list(&list, 10.0);
        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0], FRect::new(10.0, 0.0, 10.0, 10.0));
        assert_eq!(rects[1], FRect::new(0.0, 10.0, 10.0, 10.0));
        assert_eq!(rects[2], FRect::new(10.0, 10.0, 10.0, 10.0));
    }
}
