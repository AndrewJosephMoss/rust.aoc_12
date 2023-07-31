pub mod grid;

use std::collections::HashSet;

use crate::grid::coord::Coord;
use crate::grid::Grid;

pub fn process_part_1(input: &str) -> usize {
    let grid = grid::Grid::new(input);
    let start_pos = grid.get_start_coords();
    let end_pos = grid.get_end_coords();

    let steps = get_steps_to_end(&start_pos, &end_pos, &grid);
    steps
}

pub fn process_part_2(input: &str) -> usize {
    let grid = Grid::new(input);
    let start_pos = &grid.get_end_coords();
    let min_steps = get_steps_to_height(start_pos, 1, &grid);
    min_steps
}

fn get_steps_to_height(start_pos: &Coord, target_height: usize, grid: &Grid) -> usize {
    let mut visited = HashSet::from([*start_pos]);

    let mut curr_coords = vec![*start_pos];
    let mut step_coords = Vec::<Coord>::new();

    let mut iteration: usize = 0;
    let mut reached_end = false;

    while !reached_end {
        iteration += 1;
        for coord in &curr_coords {
            let steps = get_reverse_step_dirs(&coord, grid, &visited);
            if steps
                .iter()
                .any(|s| grid.get_cell_height(s) == target_height)
            {
                reached_end = true;
                break;
            }

            steps.iter().for_each(|s| {
                visited.insert(s.clone());
                step_coords.push(*s);
            });
        }
        curr_coords = step_coords.clone();
        step_coords = Vec::<Coord>::new();
    }
    iteration
}

fn get_steps_to_end(start_pos: &Coord, end_pos: &Coord, grid: &Grid) -> usize {
    let mut visited = HashSet::from([*start_pos]);

    let mut curr_coords = vec![*start_pos];
    let mut step_coords = Vec::<Coord>::new();

    let mut iteration: usize = 0;
    let mut reached_end = false;

    while !reached_end {
        iteration += 1;
        for coord in &curr_coords {
            let steps = get_step_dirs(&coord, &grid, &visited);
            if steps.contains(&end_pos) {
                reached_end = true;
                break;
            }
            steps.iter().for_each(|s| {
                visited.insert(s.clone());
                step_coords.push(*s);
            });
        }
        curr_coords = step_coords.clone();
        step_coords = Vec::<Coord>::new();
    }
    iteration
}

fn get_step_dirs(pos: &Coord, grid: &Grid, visited: &HashSet<Coord>) -> Vec<Coord> {
    let step_dirs = pos.get_step_directions();
    let step_dirs = step_dirs
        .into_iter()
        .filter(|s| s.is_in_bounds(grid.x_dim, grid.y_dim))
        .filter(|s| can_take_step(pos, &s, grid))
        .filter(|s| !visited.contains(s))
        .collect::<Vec<Coord>>();
    step_dirs
}

fn get_reverse_step_dirs(pos: &Coord, grid: &Grid, visited: &HashSet<Coord>) -> Vec<Coord> {
    let step_dirs = pos.get_step_directions();
    let step_dirs = step_dirs
        .into_iter()
        .filter(|s| s.is_in_bounds(grid.x_dim, grid.y_dim))
        .filter(|s| can_test_reverse_step(pos, &s, grid))
        .filter(|s| !visited.contains(s))
        .collect::<Vec<Coord>>();
    step_dirs
}

fn can_take_step(pos: &Coord, dest: &Coord, grid: &Grid) -> bool {
    let pos_h = grid.get_cell_height(pos);
    let dest_h = grid.get_cell_height(dest);
    pos_h >= dest_h || dest_h == pos_h + 1
}

fn can_test_reverse_step(pos: &Coord, dest: &Coord, grid: &Grid) -> bool {
    let pos_h = grid.get_cell_height(pos);
    let dest_h = grid.get_cell_height(dest);
    pos_h <= dest_h || pos_h == dest_h + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_process_part_1() {
        let result = process_part_1(TEST_INPUT);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_process_part_2() {
        let result = process_part_2(TEST_INPUT);
        assert_eq!(result, 29);
    }
}
