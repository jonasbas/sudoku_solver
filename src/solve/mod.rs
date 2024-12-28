use std::collections::VecDeque;

use crate::sudoku::Sudoku;

pub fn backtracking(sudoku_to_solve: &Sudoku) -> Option<Sudoku> {
    let mut stack: VecDeque<Sudoku> = VecDeque::new();

    stack.push_back(sudoku_to_solve.clone());

    while !stack.is_empty() {
        let current_sudoku = stack.pop_back().unwrap();

        // discard this solution and move to the next one on the stack
        if !current_sudoku.is_valid() {
            continue;
        }

        let next_empty_cell = current_sudoku.next_empty_cell();

        if next_empty_cell.is_none() {
            return Some(current_sudoku);
        }

        let (next_row, next_column) = next_empty_cell.unwrap();

        for value in 1..=9 {
            let mut next_sudoku = current_sudoku.clone();
            next_sudoku.set_value(next_row, next_column, value);

            stack.push_back(next_sudoku);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::sudoku::parse::parse_sudoku;

    use super::*;

    #[test]
    fn test_backtrack() {
        let sudoku = parse_sudoku("sudoku_files/parse_test.smd").unwrap();
        let solved = backtracking(&sudoku).unwrap();

        let expected = parse_sudoku("sudoku_files/parse_test_expected.smd").unwrap();

        assert!(solved.is_valid());
        assert_eq!(expected.values, solved.values);
    }
}
