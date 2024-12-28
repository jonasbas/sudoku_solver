use std::{fs, io::Error, path::Path};

use super::Sudoku;

pub fn sdk(path: &str) -> Result<Sudoku, Error> {
    let path = Path::new(path);

    let file_data = fs::read_to_string(path).expect("File not found.");

    // For now ignore meta data
    let data: Vec<Vec<u8>> = file_data
        .lines()
        .filter(|line| !line.starts_with("#"))
        .map(|line| {
            let data: Vec<u8> = line
                .chars()
                .map(|c| {
                    if c.is_ascii_digit() {
                        c.to_digit(10).expect(".sdk file is malformed") as u8
                    } else {
                        0
                    }
                })
                .collect();

            data
        })
        .collect();

    from_vec(&data)
}

fn from_vec(data: &[Vec<u8>]) -> Result<Sudoku, Error> {
    let rows: Vec<[u8; 9]> = data
        .iter()
        .map(|row| {
            [
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8],
            ]
        })
        .collect();

    let sudoku_data = [
        rows[0], rows[1], rows[2], rows[3], rows[4], rows[5], rows[6], rows[7], rows[8],
    ];

    let sudoku = Sudoku {
        values: sudoku_data,
    };

    Ok(sudoku)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_vec() {
        let expected: Vec<Vec<u8>> = vec![
            vec![2, 0, 0, 1, 0, 5, 0, 0, 3],
            vec![0, 5, 4, 0, 0, 0, 7, 1, 0],
            vec![0, 1, 0, 2, 0, 3, 0, 8, 0],
            vec![6, 0, 2, 8, 0, 7, 3, 0, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 5, 3, 0, 9, 8, 0, 6],
            vec![0, 2, 0, 7, 0, 1, 0, 6, 0],
            vec![0, 8, 1, 0, 0, 0, 2, 4, 0],
            vec![7, 0, 0, 4, 0, 2, 0, 0, 1],
        ];

        let parsed = from_vec(&expected).unwrap();

        assert_eq!(expected, parsed.values);
    }

    #[test]
    fn test_sdk() {
        let file_path = "src/sudoku/test_sudoku.sdk";

        let expected = [
            [2, 0, 0, 1, 0, 5, 0, 0, 3],
            [0, 5, 4, 0, 0, 0, 7, 1, 0],
            [0, 1, 0, 2, 0, 3, 0, 8, 0],
            [6, 0, 2, 8, 0, 7, 3, 0, 4],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 5, 3, 0, 9, 8, 0, 6],
            [0, 2, 0, 7, 0, 1, 0, 6, 0],
            [0, 8, 1, 0, 0, 0, 2, 4, 0],
            [7, 0, 0, 4, 0, 2, 0, 0, 1],
        ];

        let parsed = sdk(&file_path).unwrap();

        assert_eq!(expected, parsed.values);
    }
}
