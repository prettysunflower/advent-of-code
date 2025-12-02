mod inputs;

use day1::puzzle_part2;
use crate::inputs::PUZZLE_INPUT_1;

#[test]
fn test_puzzle_part2() {
    let result = puzzle_part2(&PUZZLE_INPUT_1.to_vec());
    assert_eq!(result, 6);
}

#[test]
fn test_puzzle_part2_careful_warning() {
    let result = puzzle_part2(&vec![
        "R1000"
    ]);
    assert_eq!(result, 10);
}