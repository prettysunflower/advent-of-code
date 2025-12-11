mod inputs;

use day5::puzzle_part1;
use crate::inputs::{PUZZLE_PERSONAL_INPUT, PUZZLE_QUESTION_INPUT};

#[test]
fn test_puzzle_part1_question_input() {
    let result = puzzle_part1(&PUZZLE_QUESTION_INPUT.to_vec());
    assert_eq!(result, Ok(3));
}

#[test]
fn test_puzzle_part1_personal_input() {
    let result = puzzle_part1(&PUZZLE_PERSONAL_INPUT.to_vec());
    assert_eq!(result, Ok(694));
}