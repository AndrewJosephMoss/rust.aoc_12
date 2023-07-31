use aoc_12;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let steps = aoc_12::process_part_1(&input);
    println!("Part 1: {steps}");
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let steps = aoc_12::process_part_2(&input);
    println!("Part 2: {steps}");
}
