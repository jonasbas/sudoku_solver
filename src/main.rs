use sudoku::Sudoku;

mod backtracking;
mod sudoku;

fn main() {
    let row = Sudoku {
        values: [
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
            [3, 4, 2, 1, 8, 7, 9, 6, 5],
        ],
    };

    println!("{}", row);
}
