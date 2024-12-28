pub mod parse;

use std::{collections::HashSet, fmt::Display};

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
        if idx >= 9 {
            return None;
        }

        let values = self.values;

        // can we do better?
        let data = match idx {
            0 => Some([
                values[0][0],
                values[0][1],
                values[0][2],
                values[1][0],
                values[1][1],
                values[1][2],
                values[2][0],
                values[2][1],
                values[2][2],
            ]),
            1 => Some([
                values[0][3],
                values[0][4],
                values[0][5],
                values[1][3],
                values[1][4],
                values[1][5],
                values[2][3],
                values[2][4],
                values[2][5],
            ]),
            2 => Some([
                values[0][6],
                values[0][7],
                values[0][8],
                values[1][6],
                values[1][7],
                values[1][8],
                values[2][6],
                values[2][7],
                values[2][8],
            ]),
            3 => Some([
                values[3][0],
                values[3][1],
                values[3][2],
                values[4][0],
                values[4][1],
                values[4][2],
                values[5][0],
                values[5][1],
                values[5][2],
            ]),
            4 => Some([
                values[3][3],
                values[3][4],
                values[3][5],
                values[4][3],
                values[4][4],
                values[4][5],
                values[5][3],
                values[5][4],
                values[5][5],
            ]),
            5 => Some([
                values[3][6],
                values[3][7],
                values[3][8],
                values[4][6],
                values[4][7],
                values[4][8],
                values[5][6],
                values[5][7],
                values[5][8],
            ]),
            6 => Some([
                values[6][0],
                values[6][1],
                values[6][2],
                values[7][0],
                values[7][1],
                values[7][2],
                values[8][0],
                values[8][1],
                values[8][2],
            ]),
            7 => Some([
                values[6][3],
                values[6][4],
                values[6][5],
                values[7][3],
                values[7][4],
                values[7][5],
                values[8][3],
                values[8][4],
                values[8][5],
            ]),
            8 => Some([
                values[6][6],
                values[6][7],
                values[6][8],
                values[7][6],
                values[7][7],
                values[7][8],
                values[8][6],
                values[8][7],
                values[8][8],
            ]),
            _ => None,
        };

        data.map(|x| DataRow { values: x })
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
