package day1

import "testing"
import "prettysunflower.moe/adventofcode/2025/inputs"

func TestPart1Puzzle(t *testing.T) {
	expected := 3

	result, err := part1(inputs.Day1PuzzleQuestionInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart1PuzzlePersonalInput(t *testing.T) {
	expected := 999

	result, err := part1(inputs.Day1PuzzlePersonalInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart2Puzzle(t *testing.T) {
	expected := 6

	result, err := part2(inputs.Day1PuzzleQuestionInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}

func TestPart2PuzzlePersonalInput(t *testing.T) {
	expected := 6099

	result, err := part2(inputs.Day1PuzzlePersonalInput[:])
	if err != nil {
		t.Error(err)
	}

	if result != expected {
		t.Errorf("Expected %d, got %d", expected, result)
	}
}
