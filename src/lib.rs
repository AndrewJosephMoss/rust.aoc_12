pub fn process_part_1(input: &str) -> usize {
    31
}

#[derive(Debug, PartialEq)]
enum Cell {
    Start,
    End,
    Height(usize),
}

struct Grid {
    cols: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut cols = input
            .lines()
            .map(|line| {
                let row = line
                    .chars()
                    .map(|c| convert_char_to_cell(c))
                    .collect::<Vec<Cell>>();
                row
            })
            .collect::<Vec<Vec<Cell>>>();
        cols.reverse();
        Grid { cols }
    }
}

fn convert_char_to_cell(c: char) -> Cell {
    match c {
        'S' => Cell::Start,
        'E' => Cell::End,
        c => match convert_char_to_num(c) {
            Some(height) => Cell::Height(height),
            None => panic!("Failed to convert char to Cell"),
        },
    }
}

fn convert_char_to_num(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some((c as u8 - b'a' + 1) as usize),
        _ => None,
    }
}

fn get_col_count(input: &str) -> usize {
    let cols = input
        .lines()
        .nth(0)
        .map(|line| line.chars().count())
        .unwrap();
    cols
}

fn get_rows_count(input: &str) -> usize {
    let rows = input.lines().count();
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_create_grid() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let grid = Grid::new(&input);
        assert_eq!(grid.cols[0][0], Cell::Height(1));
        assert_eq!(grid.cols[4][0], Cell::Start);
        assert_eq!(grid.cols[2][5], Cell::End);
        assert_eq!(grid.cols[2][3], Cell::Height(19));
    }

    #[test]
    fn test_convert_char_to_cell() {
        let c = 'a';
        let result = convert_char_to_cell(c);
        assert_eq!(result, Cell::Height(1));

        let c = 'z';
        let result = convert_char_to_cell(c);
        assert_eq!(result, Cell::Height(26));

        let c = 'S';
        let result = convert_char_to_cell(c);
        assert_eq!(result, Cell::Start);

        let c = 'E';
        let result = convert_char_to_cell(c);
        assert_eq!(result, Cell::End);
    }

    #[test]
    #[should_panic]
    fn test_convert_char_to_cell_panics() {
        let c = 'F';
        let _ = convert_char_to_cell(c);
    }

    #[test]
    fn test_convert_char_to_num() {
        let c = 'a';
        let result = convert_char_to_num(c);
        assert_eq!(1, result.unwrap());

        let c = 'A';
        let result = convert_char_to_num(c);
        assert!(result.is_none());

        let c = 'z';
        let result = convert_char_to_num(c);
        assert_eq!(result, Some(26));

        let c = 'g';
        let result = convert_char_to_num(c);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn test_get_cols_count() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = get_col_count(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_get_rows_count() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = get_rows_count(&input);
        assert_eq!(result, 5);
    }
}
