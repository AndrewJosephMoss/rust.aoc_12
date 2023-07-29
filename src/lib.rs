pub mod grid;

use std::borrow::BorrowMut;

use ego_tree::{NodeId, NodeMut, NodeRef, Tree};

use crate::grid::coord::Coord;
use crate::grid::Grid;

pub fn process_part_1(input: &str) -> usize {
    let grid = grid::Grid::new(input);
    31
}

fn get_routes(grid: &Grid) {
    let start = grid.get_start_coords();
    let mut routes = Tree::<Coord>::new(start.clone());
    let root = routes.root();
    let root_id = root.id();
    if traverse_grid(&start, grid, &mut routes, root_id) {
        println!("{routes:?}");
    } else {
        println!("No route found through grid");
    }
}

fn traverse_grid(pos: &Coord, grid: &Grid, routes: &mut Tree<Coord>, id: NodeId) -> bool {
    todo!()
}

fn get_step_dirs(pos: &Coord, grid: &Grid, routes: &mut Tree<Coord>, id: NodeId) -> Vec<Coord> {
    let mut step_dirs = pos.get_step_directions();
    let mut step_dirs = step_dirs
        .into_iter()
        .filter(|s| s.is_in_bounds(grid.x_dim, grid.y_dim))
        .filter(|s| can_take_step(pos, &s, grid))
        .filter(|s| !is_in_route(s, routes, id))
        .collect::<Vec<Coord>>();
    step_dirs
}

fn can_take_step(pos: &Coord, dest: &Coord, grid: &Grid) -> bool {
    let pos_h = grid.get_cell_height(pos);
    let dest_h = grid.get_cell_height(dest);
    pos_h >= dest_h || dest_h == pos_h + 1
}

fn is_in_route(pos: &Coord, routes: &mut Tree<Coord>, id: NodeId) -> bool {
    let mut node = routes
        .get(id)
        .expect(&format!("No node found at id: {id:?}"));

    if node.value() == pos {
        return true;
    }

    while let Some(parent) = node.parent() {
        if parent.value() == pos {
            return true;
        }
        node = parent;
    }
    return false;
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
    fn test_is_in_route() {
        let mut routes = Tree::<Coord>::new(Coord { x: 2, y: 2 });
        let start = routes.root_mut();
        let start_id = start.id();

        let is_in = is_in_route(&Coord { x: 2, y: 2 }, &mut routes, start_id);
        assert!(is_in);

        let mut start = routes.get_mut(start_id).unwrap();
        let node = start.append(Coord { x: 3, y: 3 });
        let node_id = node.id();

        let is_in = is_in_route(&Coord { x: 3, y: 3 }, &mut routes, node_id);
        assert!(is_in);
        let is_in = is_in_route(&Coord { x: 2, y: 2 }, &mut routes, node_id);
        assert!(is_in);
        let is_in = is_in_route(&Coord { x: 3, y: 3 }, &mut routes, start_id);
        assert!(!is_in);

        let mut node = routes.get_mut(node_id).unwrap();
        let node2 = node.append(Coord { x: 4, y: 4 });
        let node2_id = node2.id();

        let is_in = is_in_route(&Coord { x: 4, y: 4 }, &mut routes, node2_id);
        assert!(is_in);
        let is_in = is_in_route(&Coord { x: 3, y: 3 }, &mut routes, node2_id);
        assert!(is_in);
        let is_in = is_in_route(&Coord { x: 4, y: 4 }, &mut routes, node_id);
        assert!(!is_in);
    }

    #[test]
    fn test_get_routes() {
        // let grid = Grid::new(TEST_INPUT);
        // get_routes(&grid);
    }
}
