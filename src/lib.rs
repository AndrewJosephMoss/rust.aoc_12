pub mod grid;

use trees::{Node, Tree};

use crate::grid::coord::Coord;
use crate::grid::Grid;

pub fn process_part_1(input: &str) -> usize {
    let grid = grid::Grid::new(input);
    31
}

fn get_routes(grid: &Grid) {
    let start = grid.get_start_coords();
    let mut routes = Tree::<Coord>::new(start.clone());
    if traverse_grid(&start, &mut routes.root_mut()) {
    } else {
        println!("No route found through grid");
    }
}

fn traverse_grid(pos: &Coord, node: &mut Node<Coord>) -> bool {
    true
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
}
