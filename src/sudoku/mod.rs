pub mod parse;

use std::{collections::HashSet, fmt::Display};

/*
  0 1 2   3 4 5   6 7 8       box indices
0 x x x | x x x | x x x  x x x | x x x | x x x
1 x x x | x x x | x x x  x 0 x | x 1 x | x 2 x
2 x x x | x x x | x x x  x x x | x x x | x x x
  ------+-------+------  ------+-------+------
3 x x x | x x x | x x x  x x x | x x x | x x x
4 x x x | x x x | x x x  x 3 x | x 4 x | x 5 x
5 x x x | x x x | x x x  x x x | x x x | x x x
  ------+-------+------  ------+-------+------
6 x x x | x x x | x x x  x x x | x x x | x x x
7 x x x | x x x | x x x  x 6 x | x 7 x | x 8 x
8 x x x | x x x | x x x  x x x | x x x | x x x
*/
#[derive(Debug, Default, Clone)]
pub struct Sudoku {
    pub values: [[u8; 9]; 9],
}

// public methods
impl Sudoku {
    pub fn is_valid(&self) -> bool {
        for idx in 0..9 {
            let row = self.get_row(idx).expect("Sudoku row malformed.");
            let column = self.get_column(idx).expect("Sudoku column malformed.");
            let s_box = self.get_box(idx).expect("Sudoku box is malformed");

            if !row.is_valid() {
                return false;
            }
            if !column.is_valid() {
                return false;
            }
            if !s_box.is_valid() {
                return false;
            }
        }

        true
    }

    // returns pair of (row, column)
    pub fn next_empty_cell(&self) -> Option<(usize, usize)> {
        let values = self.values;

        for (i, row) in values.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value == 0 {
                    return Some((i, j));
                }
            }
        }

        None
    }

    pub fn set_value(&mut self, row: usize, column: usize, value: u8) {
        if row > 8 || column > 8 {
            return;
        }

        if value > 9 {
            return;
        }

        self.values[row][column] = value;
    }
}

// private methods
impl Sudoku {
    fn get_row(&self, idx: usize) -> Option<DataRow> {
        if idx >= 9 {
            return None;
        }

        let row = self.values[idx];
        Some(DataRow { values: row })
    }

    fn get_column(&self, idx: usize) -> Option<DataRow> {
        if idx >= 9 {
            return None;
        }

        let row: Vec<u8> = self.values.iter().map(|x| x[idx]).collect();
        let column = [
            row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8],
        ];

        Some(DataRow { values: column })
    }

    fn get_box(&self, idx: usize) -> Option<DataRow> {
        let values = self.values;

        let offset = match idx {
            0 => (0, 0),
            1 => (0, 3),
            2 => (0, 6),
            3 => (3, 0),
            4 => (3, 3),
            5 => (3, 6),
            6 => (6, 0),
            7 => (6, 3),
            8 => (6, 6),
            _ => return None,
        };

        let box_indices = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];

        let data = box_indices.map(|(x, y)| {
            let x = x + offset.1;
            let y = y + offset.1;

            values[x][y]
        });

        Some(DataRow { values: data })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct DataRow {
    values: [u8; 9],
}

impl DataRow {
    fn is_valid(&self) -> bool {
        let values_without_zero = self.values.iter().filter(|x| **x != 0).collect::<Vec<_>>();

        let expected_length = values_without_zero.len();

        let distinct_length = values_without_zero.iter().collect::<HashSet<_>>().len();
        expected_length == distinct_length
    }
}

// display implementations
impl Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {} | {} {} {} | {} {} {}",
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
            self.values[4],
            self.values[5],
            self.values[6],
            self.values[7],
            self.values[8],
        )
    }
}
impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in 0..9 {
            if idx == 3 || idx == 6 {
                writeln!(f, "------+-------+------")?;
            }

            let row = self.get_row(idx).expect("Sudoku is malformed.");
            row.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_row() {
        // row is valid
        let row = DataRow {
            values: [3, 4, 2, 1, 8, 7, 9, 6, 5],
        };
        assert!(row.is_valid(), "Should be valid row.");

        // row has duplicates
        let row = DataRow {
            values: [3, 3, 2, 1, 8, 7, 9, 6, 5],
        };
        assert!(!row.is_valid(), "Should be invalid due to duplicates.");
    }

    #[test]
    fn test_get_box() {
        let test_sudoku = Sudoku {
            values: [
                [2, 0, 0, 1, 0, 5, 0, 0, 3],
                [0, 5, 4, 0, 0, 0, 7, 1, 0],
                [0, 1, 0, 2, 0, 3, 0, 8, 0],
                [6, 0, 2, 8, 0, 7, 3, 0, 4],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 5, 3, 0, 9, 8, 0, 6],
                [0, 2, 0, 7, 0, 1, 0, 6, 0],
                [0, 8, 1, 0, 0, 0, 2, 4, 0],
                [7, 0, 0, 4, 0, 2, 0, 0, 1],
            ],
        };

        let expected = [2, 0, 0, 0, 5, 4, 0, 1, 0];
        assert_eq!(expected, test_sudoku.get_box(0).unwrap().values);

        let expected = [8, 0, 7, 0, 0, 0, 3, 0, 9];
        assert_eq!(expected, test_sudoku.get_box(4).unwrap().values);

        let expected = [0, 6, 0, 2, 4, 0, 0, 0, 1];
        assert_eq!(expected, test_sudoku.get_box(8).unwrap().values);
    }

    #[test]
    fn test_next_empty_cell() {
        let test_sudoku = Sudoku {
            values: [
                [2, 0, 0, 1, 0, 5, 0, 0, 3],
                [0, 5, 4, 0, 0, 0, 7, 1, 0],
                [0, 1, 0, 2, 0, 3, 0, 8, 0],
                [6, 0, 2, 8, 0, 7, 3, 0, 4],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 5, 3, 0, 9, 8, 0, 6],
                [0, 2, 0, 7, 0, 1, 0, 6, 0],
                [0, 8, 1, 0, 0, 0, 2, 4, 0],
                [7, 0, 0, 4, 0, 2, 0, 0, 1],
            ],
        };

        assert_eq!((0, 1), test_sudoku.next_empty_cell().unwrap());

        let test_sudoku = Sudoku {
            values: [
                [2, 3, 6, 1, 4, 5, 9, 7, 3],
                [0, 5, 4, 0, 0, 0, 7, 1, 0],
                [0, 1, 0, 2, 0, 3, 0, 8, 0],
                [6, 0, 2, 8, 0, 7, 3, 0, 4],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 5, 3, 0, 9, 8, 0, 6],
                [0, 2, 0, 7, 0, 1, 0, 6, 0],
                [0, 8, 1, 0, 0, 0, 2, 4, 0],
                [7, 0, 0, 4, 0, 2, 0, 0, 1],
            ],
        };

        assert_eq!((1, 0), test_sudoku.next_empty_cell().unwrap());
    }

    #[test]
    fn test_set_value() {
        let mut test_sudoku = Sudoku {
            values: [
                [2, 3, 6, 1, 4, 5, 9, 7, 3],
                [0, 5, 4, 0, 0, 0, 7, 1, 0],
                [0, 1, 0, 2, 0, 3, 0, 8, 0],
                [6, 0, 2, 8, 0, 7, 3, 0, 4],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 5, 3, 0, 9, 8, 0, 6],
                [0, 2, 0, 7, 0, 1, 0, 6, 0],
                [0, 8, 1, 0, 0, 0, 2, 4, 0],
                [7, 0, 0, 4, 0, 2, 0, 0, 1],
            ],
        };

        test_sudoku.set_value(1, 0, 8);
        let expected_sudoku = Sudoku {
            values: [
                [2, 3, 6, 1, 4, 5, 9, 7, 3],
                [8, 5, 4, 0, 0, 0, 7, 1, 0],
                [0, 1, 0, 2, 0, 3, 0, 8, 0],
                [6, 0, 2, 8, 0, 7, 3, 0, 4],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 5, 3, 0, 9, 8, 0, 6],
                [0, 2, 0, 7, 0, 1, 0, 6, 0],
                [0, 8, 1, 0, 0, 0, 2, 4, 0],
                [7, 0, 0, 4, 0, 2, 0, 0, 1],
            ],
        };

        assert_eq!(expected_sudoku.values, test_sudoku.values);
    }
}
