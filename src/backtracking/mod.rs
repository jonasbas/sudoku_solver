use std::collections::VecDeque;

use crate::sudoku::Sudoku;

pub fn solve(sudoku_to_solve: &Sudoku) -> Option<Sudoku> {
    let mut stack: VecDeque<Sudoku> = VecDeque::new();

    stack.push_back(sudoku_to_solve.clone());

    while !stack.is_empty() {
        let current_state = stack.pop_back().unwrap();

        let sudoku = current_state;

        // discard this solution and move the next one on the stack
        if !sudoku.is_valid() {
            continue;
        }

        let next_empty_cell = sudoku.next_empty_cell();

        if next_empty_cell.is_none() {
            return Some(sudoku);
        }

        let (next_row, next_column) = next_empty_cell.unwrap();

        for value in 1..=9 {
            let mut next_sudoku = sudoku.clone();
            next_sudoku.set_value(next_row, next_column, value);

            stack.push_back(next_sudoku);
        }
    }

    None
}
