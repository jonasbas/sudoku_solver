use sudoku::parse::sdk;

mod backtracking;
mod sudoku;

fn main() {
    let sudoku = sdk("src/sudoku/test_sudoku.sdk").unwrap();

    println!("Input:");
    println!("{}", sudoku);
}
