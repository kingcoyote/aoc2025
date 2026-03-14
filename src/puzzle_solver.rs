// src/puzzle_solver.rs

pub struct SolverOptions<'a> {
    pub input: &'a str,
    pub verbose: bool,
    pub debug: bool,
}

pub fn pause_if_debug(debug: bool, msg: &str) {
    if debug {
        println!("[DEBUG] {} Press Enter to continue...", msg);
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

pub trait PuzzleSolver {
    fn solve(&self, options: &SolverOptions) -> i64;
}

pub struct Puzzle1;
impl PuzzleSolver for Puzzle1 {
    fn solve(&self, options: &SolverOptions) -> i64 {
        pause_if_debug(options.debug, "Paused at Puzzle 1.");
        // ... actual logic here ...
        1
    }
}

pub struct Puzzle2;
impl PuzzleSolver for Puzzle2 {
    fn solve(&self, options: &SolverOptions) -> i64 {
        pause_if_debug(options.debug, "Paused at Puzzle 2.");
        // ... actual logic here ...
        2
    }
}

// ... Add more puzzles as needed ...

pub fn get_solver(puzzle: u8) -> Option<Box<dyn PuzzleSolver>> {
    match puzzle {
        1 => Some(Box::new(Puzzle1)),
        2 => Some(Box::new(Puzzle2)),
        // ... add more as needed ...
        _ => None,
    }
}
