use std::{
    collections::HashSet,
    fmt::{Display, Write},
    usize,
};

#[derive(Debug, Default)]
pub struct DataRow {
    pub values: [u8; 9],
}

impl DataRow {
    fn is_valid(&self) -> bool {
        let any_zero = self.values.iter().any(|x| *x == 0);

        if any_zero {
            return false;
        }

        let distinct_length = self.values.iter().collect::<HashSet<_>>().len();
        9 == distinct_length
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
  0 1 2   3 4 5   6 7 8
0 x x x | x x x | x x x
1 x x x | x x x | x x x
2 x x x | x x x | x x x
  ------+-------+------
3 x x x | x x x | x x x
4 x x x | x x x | x x x
5 x x x | x x x | x x x
  ------+-------+------
6 x x x | x x x | x x x
7 x x x | x x x | x x x
8 x x x | x x x | x x x
*/
#[derive(Debug, Default)]
pub struct Sudoku {
    pub values: [[u8; 9]; 9],
}

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
