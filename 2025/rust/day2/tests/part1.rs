mod inputs;

use day2::puzzle_part1;
use crate::inputs::{PUZZLE_QUESTION_INPUT, PUZZLE_PERSONAL_INPUT};

#[test]
fn test_puzzle_part1_question_input() {
    let result = puzzle_part1(PUZZLE_QUESTION_INPUT);
    assert_eq!(result, 1227775554);
}

#[test]
fn test_puzzle_part1_personal_input() {
    let result = puzzle_part1(PUZZLE_PERSONAL_INPUT);
    assert_eq!(result, 31210613313);
}