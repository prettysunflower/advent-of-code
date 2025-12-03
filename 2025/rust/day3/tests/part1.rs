mod inputs;

use day3::puzzle;
use crate::inputs::{PUZZLE_PERSONAL_INPUT, PUZZLE_QUESTION_INPUT};

#[test]
fn test_puzzle_part1_question_input() {
    let result = puzzle(&PUZZLE_QUESTION_INPUT.to_vec(), 2);
    assert_eq!(result, 357);
}

#[test]
fn test_puzzle_3_limit() {
    let result = puzzle(&vec!["818181911112111"], 3);
    assert_eq!(result, 921);
}
#[test]
fn test_puzzle_part1_personal_puzzle_input() {
    let result = puzzle(&PUZZLE_PERSONAL_INPUT.to_vec(), 2);
    assert_eq!(result, 17343);
}