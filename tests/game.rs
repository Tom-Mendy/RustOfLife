#[cfg(test)]
mod tests {

    use rust_of_life::game::{Game, GameStatus};

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.get_name(), "Rust Of Life");
        assert_eq!(game.get_game_state(), GameStatus::Pause);
        assert_eq!(game.get_size_grid(), 100);
        assert_eq!(game.get_window_height(), 1000);
        assert_eq!(game.get_window_width(), 1000);
        assert_eq!(game.get_window_min_length(), 1000);
        assert_eq!(game.get_unit_grid(), 10.0);
        assert_eq!(game.get_iteration(), 0);
        assert_eq!(game.get_max_iteration_per_second(), 10);
    }

    #[test]
    fn test_set_game_state() {
        let mut game = Game::new();
        game.set_game_state(GameStatus::Running);
        assert_eq!(game.get_game_state(), GameStatus::Running);
    }

    #[test]
    fn test_set_window_height() {
        let mut game = Game::new();
        game.set_window_height(800);
        assert_eq!(game.get_window_height(), 800);
        assert_eq!(game.get_unit_grid(), 8.0);
        assert_eq!(game.get_window_min_length(), 800);
    }

    #[test]
    fn test_set_window_width() {
        let mut game = Game::new();
        game.set_window_width(800);
        assert_eq!(game.get_window_width(), 800);
        assert_eq!(game.get_unit_grid(), 8.0);
        assert_eq!(game.get_window_min_length(), 800);
    }

    #[test]
    fn test_run_game() {
        let mut game = Game::new();
        game.run();
        assert_eq!(game.get_game_state(), GameStatus::Running);
        assert_eq!(game.get_start_time_iteration(), 0);
    }

    #[test]
    fn test_reset_game() {
        let mut game = Game::new();
        game.run();
        game.reset();
        assert_eq!(game.get_iteration(), 0);
        assert_eq!(game.get_start_time_iteration(), 0);
        assert_eq!(game.get_game_state(), GameStatus::Pause);
    }

    #[test]
    fn test_get_iteration_per_second() {
        let mut game = Game::new();
        game.run();
        game.set_iteration(100);
        let ips = game.get_iteration_per_second();
        assert!(ips >= 0.0);
    }
}
