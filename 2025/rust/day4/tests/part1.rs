mod inputs;

use day4::puzzle;
use crate::inputs::{PUZZLE_PERSONAL_INPUT, PUZZLE_QUESTION_INPUT};

#[test]
fn test_puzzle_part1_question_input() {
    let (result, output) = puzzle(PUZZLE_QUESTION_INPUT.into_iter().map(|input| {input.to_string()}).collect());
    println!("{}", output.join("\n"));
    assert_eq!(result, 13);
}

#[test]
fn test_puzzle_part1_personal_puzzle_input() {
    let (result, output) = puzzle(PUZZLE_PERSONAL_INPUT.into_iter().map(|input| {input.to_string()}).collect());
    println!("{}", output.join("\n"));
    assert_eq!(result, 1395);
}