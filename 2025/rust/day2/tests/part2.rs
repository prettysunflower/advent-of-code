mod inputs;

use day2::puzzle_part2;
use crate::inputs::{PUZZLE_QUESTION_INPUT, PUZZLE_PERSONAL_INPUT};

#[test]
fn test_puzzle_part2_question_input() {
    let result = puzzle_part2(PUZZLE_QUESTION_INPUT);
    assert_eq!(result, 4174379265);
}

#[test]
fn test_puzzle_part1_personal_input() {
    let result = puzzle_part2(PUZZLE_PERSONAL_INPUT);
    assert_eq!(result, 41823587546);
}