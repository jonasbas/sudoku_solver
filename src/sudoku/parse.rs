use std::{fs, io::Error, path::Path};

use super::Sudoku;

pub fn parse_sudoku(path: &str) -> Result<Sudoku, Error> {
    let path = Path::new(path);
    let extension = path.extension();

    if extension.is_none() {
        return Err(Error::new(
            std::io::ErrorKind::Unsupported,
            "file path is malformed or has no supported file extension.",
        ));
    }

    let extension = extension.unwrap();

    let result = match extension.to_str() {
        Some("sdk") => sdk(path),
        Some("smd") => smd(path),
        _ => Err(Error::new(
            std::io::ErrorKind::Unsupported,
            "extension type is not supported.",
        )),
    };

    result
}

fn sdk(path: &Path) -> Result<Sudoku, Error> {
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

// currently supports only one puzzle should support multiple in the future
fn smd(path: &Path) -> Result<Sudoku, Error> {
    let file_data = fs::read_to_string(path).expect("File not found.");

    let mut data: [[u8; 9]; 9] = [[0; 9]; 9];
    for (idx, c) in file_data.chars().enumerate() {
        if idx > 80 {
            break;
        }

        let row = idx / 9;
        let column = idx % 9;

        data[row][column] = c.to_digit(10).expect("sudoku file is malformed") as u8;
    }

    Ok(Sudoku { values: data })
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
        let file_path = Path::new("sudoku_files/test_sudoku.sdk");

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

        let parsed = sdk(file_path).unwrap();

        assert_eq!(expected, parsed.values);
    }

    #[test]
    fn test_smd() {
        let file_path = Path::new("sudoku_files/test_sudoku.smd");

        let expected = [
            [0, 0, 0, 4, 0, 0, 0, 9, 8],
            [0, 0, 0, 5, 0, 9, 0, 1, 2],
            [0, 0, 0, 0, 3, 0, 0, 0, 0],
            [8, 0, 9, 0, 0, 4, 5, 0, 0],
            [0, 5, 2, 1, 8, 0, 0, 4, 0],
            [0, 0, 0, 0, 0, 0, 0, 3, 0],
            [9, 6, 0, 8, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 2, 0, 0, 0, 0],
            [0, 0, 8, 9, 0, 0, 6, 0, 0],
        ];

        let parsed = smd(file_path).unwrap();

        assert_eq!(expected, parsed.values);
    }
}
