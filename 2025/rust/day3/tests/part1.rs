mod inputs;

use day3::puzzle_part1;
use crate::inputs::{PUZZLE_QUESTION_INPUT};

#[test]
fn test_puzzle_part1_question_input() {
    let result = puzzle_part1(&PUZZLE_QUESTION_INPUT.to_vec());
    assert_eq!(result, 357);
}