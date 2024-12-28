use sudoku::parse::parse_sudoku;

mod backtracking;
mod sudoku;

fn main() {
    let sudoku = parse_sudoku("src/sudoku/test_sudoku.sdk").unwrap();

    println!("Input:");
    println!("{}", sudoku);

    let solved = backtracking::solve(&sudoku).expect("No Solution found");

    println!("Result:");
    println!("{}", solved);
}
