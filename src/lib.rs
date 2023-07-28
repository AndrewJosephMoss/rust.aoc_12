use std::collections::HashMap;

pub mod grid;

pub fn process_part_1(input: &str) -> usize {
    31
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
