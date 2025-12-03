mod inputs;

use day3::puzzle;
use crate::inputs::{PUZZLE_QUESTION_INPUT, PUZZLE_PERSONAL_INPUT};

#[test]
fn test_puzzle_part2_question_input() {
    let result = puzzle(&PUZZLE_QUESTION_INPUT.to_vec(), 12);
    assert_eq!(result, 3121910778619);
}

#[test]
fn test_puzzle_part1_personal_input() {
    let result = puzzle(&PUZZLE_PERSONAL_INPUT.to_vec(), 12);
    assert_eq!(result, 172664333119298);
}