mod inputs;

use day1::puzzle_part1;
use crate::inputs::{PUZZLE_INPUT_1, PUZZLE_INPUT_2};

#[test]
fn test_puzzle_part1() {
    let result = puzzle_part1(&PUZZLE_INPUT_1.to_vec());
    assert_eq!(result, 3);
}

#[test]
fn test_puzzle_part1_self_puzzle_input() {
    let result = puzzle_part1(&PUZZLE_INPUT_2.to_vec());
    assert_eq!(result, 999);
}