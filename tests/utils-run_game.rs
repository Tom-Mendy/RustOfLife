#[cfg(test)]
mod tests {

    use rust_of_life::utils::run_game;

    #[test]
    fn test_run_game() {
        std::thread::spawn(|| {
            let _ = run_game();
        });
    }
}
