use std::cmp::min;

/// Represents the current status of the game.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum GameStatus {
    /// The game is exiting.
    Exit,
    /// The game is paused.
    #[default]
    Pause,
    /// The game is running.
    Running,
}

/// Main game state and configuration.
#[derive(Debug, Default, Clone)]
pub struct Game {
    name: String,
    game_state: GameStatus,
    size_grid: u32,
    window_height: u32,
    window_width: u32,
    window_min_length: u32,
    unit_grid: f32,
    iteration: u32,
    start_time: chrono::DateTime<chrono::Local>,
    start_time_iteration: u32,
    max_iteration_per_second: u32,
}

impl Game {
    /// Creates a new game instance with default settings.
    pub fn new() -> Self {
        let mut new_instance = Self {
            name: "Rust Of Life".to_string(),
            game_state: GameStatus::Pause,
            size_grid: 100,
            window_height: 1000,
            window_width: 1000,
            window_min_length: 1000,
            unit_grid: 0.0,
            iteration: 0,
            start_time: chrono::Local::now(),
            start_time_iteration: 0,
            max_iteration_per_second: 10,
        };
        new_instance.calculate_unit_grid();
        new_instance
    }

    fn calculate_unit_grid(&mut self) {
        self.unit_grid = self.window_min_length as f32 / self.size_grid as f32;
    }

    /// Returns the number of iterations per second since the game started.
    pub fn get_iteration_per_second(&self) -> f64 {
        let n1 = (self.get_iteration() - self.get_start_time_iteration()) as f64
            / (chrono::Local::now() - self.get_start_time()).num_seconds() as f64;
        (n1 * 10.0).trunc() / 10.0
    }

    /// Returns the name of the game.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the current game state.
    pub fn get_game_state(&self) -> GameStatus {
        self.game_state.clone()
    }

    /// Returns the size of the grid.
    pub fn get_size_grid(&self) -> u32 {
        self.size_grid
    }

    /// Returns the window height.
    pub fn get_window_height(&self) -> u32 {
        self.window_height
    }

    /// Returns the window width.
    pub fn get_window_width(&self) -> u32 {
        self.window_width
    }

    /// Returns the size of a grid unit.
    pub fn get_unit_grid(&self) -> f32 {
        self.unit_grid
    }

    /// Returns the current iteration count.
    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }

    /// Returns the start time of the game.
    pub fn get_start_time(&self) -> chrono::DateTime<chrono::Local> {
        self.start_time
    }

    /// Returns the iteration count at the start time.
    pub fn get_start_time_iteration(&self) -> u32 {
        self.start_time_iteration
    }

    /// Returns the maximum allowed iterations per second.
    pub fn get_max_iteration_per_second(&self) -> u32 {
        self.max_iteration_per_second
    }

    /// Returns the minimum window length.
    pub fn get_window_min_length(&self) -> u32 {
        self.window_min_length
    }

    /// Sets the current game state.
    pub fn set_game_state(&mut self, game_state: GameStatus) {
        self.game_state = game_state;
    }

    /// Sets the grid size and updates the unit grid.
    pub fn set_size_grid(&mut self, size_grid: u32) {
        self.size_grid = size_grid;
    }

    /// Sets the window height.
    pub fn set_window_height(&mut self, window_height: u32) {
        self.window_height = window_height;
        self.window_min_length = min(self.window_height, self.window_width);
        self.calculate_unit_grid();
    }

    /// Sets the window width.
    pub fn set_window_width(&mut self, window_width: u32) {
        self.window_width = window_width;
        self.window_min_length = min(self.window_height, self.window_width);
        self.calculate_unit_grid();
    }

    /// Sets the unit grid size.
    pub fn set_unit_grid(&mut self, unit_grid: f32) {
        self.unit_grid = unit_grid;
    }

    /// Sets the current iteration count.
    pub fn set_iteration(&mut self, iteration: u32) {
        self.iteration = iteration;
    }

    /// Sets the start time of the game.
    pub fn set_start_time(&mut self, start_time: chrono::DateTime<chrono::Local>) {
        self.start_time = start_time;
    }

    /// Sets the iteration count at the start time.
    pub fn set_start_time_iteration(&mut self, start_time_iteration: u32) {
        self.start_time_iteration = start_time_iteration;
    }

    /// Runs the main game logic for one frame.
    pub fn run(&mut self) {
        self.set_game_state(GameStatus::Running);
        self.set_start_time(chrono::Local::now());
        self.set_start_time_iteration(self.get_iteration());
    }

    /// Resets the game to its initial state.
    pub fn reset(&mut self) {
        self.set_iteration(0);
        self.set_start_time(chrono::Local::now());
        self.set_start_time_iteration(0);
        self.set_game_state(GameStatus::Pause);
    }
}
