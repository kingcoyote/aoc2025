
mod puzzle_solver;
use puzzle_solver::{get_solver, SolverOptions};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about = "Advent of Code 2025 CLI", long_about = None)]
struct Args {
    /// Puzzle number to solve (1-12)
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=12))]
    puzzle: u8,

    /// Input file name
    #[arg(long)]
    input: String,

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,

    /// Enable debug mode (pauses execution at breakpoints)
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let solver = match get_solver(args.puzzle) {
        Some(s) => s,
        None => {
            eprintln!("No solver implemented for puzzle {}", args.puzzle);
            std::process::exit(1);
        }
    };

    let options = SolverOptions {
        input: &args.input,
        verbose: args.verbose,
        debug: args.debug,
    };

    let result = solver.solve(&options);
    println!("Result: {}", result);
}
