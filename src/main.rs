use sudoku::parse::parse_sudoku;

mod solve;
mod sudoku;
fn main() {
    let sudoku = parse_sudoku("sudoku_files/test_sudoku.sdk").unwrap();

    println!("Input:");
    println!("{}", sudoku);

    let solved = solve::backtracking(&sudoku).expect("No Solution found");

    println!("Result:");
    println!("{}", solved);
}
