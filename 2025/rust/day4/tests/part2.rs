mod inputs;

use crate::inputs::{PUZZLE_PERSONAL_INPUT, PUZZLE_QUESTION_INPUT};
use day4::puzzle_part2;

#[test]
fn test_puzzle_part2_question_input() {
    let (result, output) = puzzle_part2(PUZZLE_QUESTION_INPUT.into_iter().map(|input| {input.to_string()}).collect());
    println!("{}", output.join("\n"));
    assert_eq!(result, 43);
}

#[test]
fn test_puzzle_part2_personal_puzzle_input() {
    let (result, output) = puzzle_part2(PUZZLE_PERSONAL_INPUT.into_iter().map(|input| {input.to_string()}).collect());
    println!("{}", output.join("\n"));
    assert_eq!(result, 8451);
}