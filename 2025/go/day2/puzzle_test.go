package day2

import "testing"
import "prettysunflower.moe/adventofcode/2025/inputs"

func TestPart1Puzzle(t *testing.T) {
	expected := 1227775554

	result, err := part1(inputs.Day2PuzzleQuestionInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart1PuzzlePersonalInput(t *testing.T) {
	expected := 31210613313

	result, err := part1(inputs.Day2PuzzlePersonalInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart2Puzzle(t *testing.T) {
	expected := 4174379265

	result, err := part2(inputs.Day2PuzzleQuestionInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart2PuzzlePersonalInput(t *testing.T) {
	expected := 41823587546

	result, err := part2(inputs.Day2PuzzlePersonalInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
