pub mod game {

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameStatus {
        Exit,
        Pause,
        Running,
    }

    #[derive(Debug, Clone)]
    pub struct Game {
        name: String,
        game_state: GameStatus,
        size_grid: u32,
        window_height: u32,
        window_width: u32,
        unit_grid: f32,
        iteration: u32,
        start_time: chrono::DateTime<chrono::Local>,
        start_time_iteration: u32,
        max_iteration_per_second: u32,
    }

    impl Game {
        pub fn new() -> Self {
            let mut new_instance = Self {
                name: "Rust Of Life".to_string(),
                game_state: GameStatus::Pause,
                size_grid: 100,
                window_height: 1000,
                window_width: 1000,
                unit_grid: 0.0,
                iteration: 0,
                start_time: chrono::Local::now(),
                start_time_iteration: 0,
                max_iteration_per_second: 10,
            };
            new_instance.calculate_unit_grid();
            return new_instance;
        }

        fn calculate_unit_grid(&mut self) {
            self.unit_grid = self.window_width as f32 / self.size_grid as f32;
        }

        pub fn get_iteration_per_second(&self) -> f64 {
            let n1 = (self.get_iteration() - self.get_start_time_iteration()) as f64
                / (chrono::Local::now() - self.get_start_time()).num_seconds() as f64;
            (n1 * 10.0).trunc() / 10.0
        }

        pub fn get_name(&self) -> String {
            return self.name.clone();
        }

        pub fn get_game_state(&self) -> GameStatus {
            return self.game_state.clone();
        }

        pub fn get_size_grid(&self) -> u32 {
            return self.size_grid.clone();
        }

        pub fn get_window_height(&self) -> u32 {
            return self.window_height.clone();
        }

        pub fn get_window_width(&self) -> u32 {
            return self.window_width;
        }

        pub fn get_unit_grid(&self) -> f32 {
            return self.unit_grid;
        }

        pub fn get_iteration(&self) -> u32 {
            return self.iteration;
        }

        pub fn get_start_time(&self) -> chrono::DateTime<chrono::Local> {
            return self.start_time;
        }

        pub fn get_start_time_iteration(&self) -> u32 {
            return self.start_time_iteration;
        }

        pub fn get_max_iteration_per_second(&self) -> u32 {
            return self.max_iteration_per_second;
        }

        pub fn set_game_state(&mut self, game_state: GameStatus) {
            self.game_state = game_state;
        }

        pub fn set_size_grid(&mut self, size_grid: u32) {
            self.size_grid = size_grid;
        }

        pub fn set_window_height(&mut self, window_height: u32) {
            self.window_height = window_height;
            self.calculate_unit_grid()
        }

        pub fn set_window_width(&mut self, window_width: u32) {
            self.window_width = window_width;
            self.calculate_unit_grid()
        }

        pub fn set_unit_grid(&mut self, unit_grid: f32) {
            self.unit_grid = unit_grid;
        }

        pub fn set_iteration(&mut self, iteration: u32) {
            self.iteration = iteration;
        }

        pub fn set_start_time(&mut self, start_time: chrono::DateTime<chrono::Local>) {
            self.start_time = start_time;
        }

        pub fn set_start_time_iteration(&mut self, start_time_iteration: u32) {
            self.start_time_iteration = start_time_iteration;
        }

        pub fn run(&mut self) {
            self.set_game_state(GameStatus::Running);
            self.set_start_time(chrono::Local::now());
            self.set_start_time_iteration(self.get_iteration());
        }

        pub fn reset(&mut self) {
            self.set_iteration(0);
            self.set_start_time(chrono::Local::now());
            self.set_start_time_iteration(0);
            self.set_game_state(GameStatus::Pause);
        }
    }
}
