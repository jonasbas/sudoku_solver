use sudoku::parse::{sdk, smd};

mod backtracking;
mod sudoku;

fn main() {
    let sudoku = smd("src/sudoku/test_sudoku.smd").unwrap();

    println!("Input:");
    println!("{}", sudoku);

    let solved = backtracking::solve(&sudoku).expect("No Solution found");

    println!("Result:");
    println!("{}", solved);
}
