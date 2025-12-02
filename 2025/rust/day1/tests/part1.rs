mod inputs;

use day1::puzzle_part1;
use crate::inputs::{PUZZLE_QUESTION_INPUT, PUZZLE_PERSONAL_INPUT};

#[test]
fn test_puzzle_part1() {
    let result = puzzle_part1(&PUZZLE_QUESTION_INPUT.to_vec());
    assert_eq!(result, 3);
}

#[test]
fn test_puzzle_part1_personal_puzzle_input() {
    let result = puzzle_part1(&PUZZLE_PERSONAL_INPUT.to_vec());
    assert_eq!(result, 999);
}