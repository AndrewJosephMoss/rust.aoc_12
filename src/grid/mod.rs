pub mod coord;
use self::coord::Coord;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Start,
    End,
    Height(usize),
}

pub struct Grid {
    cols: Vec<Vec<Cell>>,
    x_dim: usize,
    y_dim: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut cols = input
            .lines()
            .map(|line| {
                let row = line
                    .chars()
                    .map(|c| Grid::convert_char_to_cell(c))
                    .collect::<Vec<Cell>>();
                row
            })
            .collect::<Vec<Vec<Cell>>>();
        cols.reverse();
        let y_dim = cols.len();
        let x_dim = cols[0].len();
        Grid { cols, y_dim, x_dim }
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

    fn get_end_coords(&self) -> Coord {
        for y in 0..self.cols.len() {
            for x in 0..self.cols[y].len() {
                if self.cols[y][x] == Cell::End {
                    return Coord { y, x };
                }
            }
        }
        panic!("Failed to find starting coord");
    }

    fn get_cell(&self, coord: &Coord) -> &Cell {
        &self.cols[coord.y][coord.x]
    }

    fn get_cell_height(&self, coord: &Coord) -> usize {
        let pos_cell = self.get_cell(coord);
        match pos_cell {
            Cell::Start => 1,
            Cell::End => 26,
            Cell::Height(h) => *h,
        }
    }

    fn convert_char_to_cell(c: char) -> Cell {
        match c {
            'S' => Cell::Start,
            'E' => Cell::End,
            c => match Grid::convert_char_to_num(c) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test-input.txt");

    #[test]
    fn test_create_grid() {
        let grid = Grid::new(TEST_INPUT);
        assert_eq!(grid.cols[0][0], Cell::Height(1));
        assert_eq!(grid.cols[4][0], Cell::Start);
        assert_eq!(grid.cols[2][5], Cell::End);
        assert_eq!(grid.cols[2][3], Cell::Height(19));
    }

    #[test]
    fn test_get_cell_height() {
        let grid = Grid::new(TEST_INPUT);

        let coord = Coord { x: 0, y: 4 };
        let height = grid.get_cell_height(&coord);
        assert_eq!(height, 1);
    }

    #[test]
    fn test_get_start_coord() {
        let grid = Grid::new(TEST_INPUT);
        let start = grid.get_start_coords();
        assert_eq!(start, Coord { x: 0, y: 4 });
    }

    #[test]
    fn test_get_end_coord() {
        let grid = Grid::new(TEST_INPUT);
        let end = grid.get_end_coords();
        assert_eq!(end, Coord { x: 5, y: 2 });
    }

    #[test]
    fn get_cell() {
        let grid = Grid::new(TEST_INPUT);
        let cell = grid.get_cell(&Coord { x: 0, y: 3 });
        assert_eq!(*cell, Cell::Height(1));

        let cell = grid.get_cell(&Coord { x: 2, y: 2 });
        assert_eq!(*cell, Cell::Height(3));

        let cell = grid.get_cell(&Coord { x: 5, y: 3 });
        assert_eq!(*cell, Cell::Height(24));
    }

    #[test]
    fn test_convert_char_to_cell() {
        let c = 'a';
        let result = Grid::convert_char_to_cell(c);
        assert_eq!(result, Cell::Height(1));

        let c = 'z';
        let result = Grid::convert_char_to_cell(c);
        assert_eq!(result, Cell::Height(26));

        let c = 'S';
        let result = Grid::convert_char_to_cell(c);
        assert_eq!(result, Cell::Start);

        let c = 'E';
        let result = Grid::convert_char_to_cell(c);
        assert_eq!(result, Cell::End);
    }

    #[test]
    #[should_panic]
    fn test_convert_char_to_cell_panics() {
        let c = 'F';
        let _ = Grid::convert_char_to_cell(c);
    }

    #[test]
    fn test_convert_char_to_num() {
        let c = 'a';
        let result = Grid::convert_char_to_num(c);
        assert_eq!(1, result.unwrap());

        let c = 'A';
        let result = Grid::convert_char_to_num(c);
        assert!(result.is_none());

        let c = 'z';
        let result = Grid::convert_char_to_num(c);
        assert_eq!(result, Some(26));

        let c = 'g';
        let result = Grid::convert_char_to_num(c);
        assert_eq!(result.unwrap(), 7);
    }
}
