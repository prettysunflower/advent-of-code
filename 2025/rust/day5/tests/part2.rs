mod inputs;

use day5::puzzle_part2;
use crate::inputs::{PUZZLE_PERSONAL_INPUT, PUZZLE_QUESTION_INPUT};

#[test]
fn test_puzzle_part2_question_input() {
    let result = puzzle_part2(&PUZZLE_QUESTION_INPUT.to_vec());
    assert_eq!(result, Ok(14));
}

#[test]
fn test_puzzle_part2_test_if_one_number() {
    let result = puzzle_part2(&vec!["557257867290147-557257867290147", ""]);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_puzzle_part2_test_if_two_numbers() {
    let result = puzzle_part2(&vec!["557257867290147-557257867290147", "557257867290148-557257867290148", ""]);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_puzzle_part2_personal_input() {
    let result = puzzle_part2(&PUZZLE_PERSONAL_INPUT.to_vec());
    assert_eq!(result, Ok(352716206375547));
}