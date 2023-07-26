use std::collections::HashMap;

pub fn process_part_1(input: &str) -> usize {
    let grid = Grid::new(input);
    31
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Start,
    End,
    Height(usize),
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn get_step_directions(&self) -> [Coord; 4] {
        [
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }

    pub fn is_in_bounds(&self, x_dim: usize, y_dim: usize) -> bool {
        self.x < x_dim && self.y < y_dim
    }
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

    fn get_start_coords(&self) -> Coord {
        for y in 0..self.cols.len() {
            for x in 0..self.cols[y].len() {
                if self.cols[y][x] == Cell::Start {
                    return Coord { y, x };
                }
            }
        }
        panic!("Failed to find starting coord");
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        &self.cols[coord.y][coord.x]
    }

    fn find_min_path_length() -> usize {
        todo!()
    }

    fn take_step(&self, pos: &Coord, visited: &HashMap<Coord, usize>) -> usize {
        if *self.get_cell(pos) == Cell::End {
            return 1;
        }

        if visited.contains_key(&pos) {
            return *visited.get(&pos).unwrap();
        }

        todo!()
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
