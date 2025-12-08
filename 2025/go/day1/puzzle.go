package day1

import (
	"errors"
	"strconv"
)

func part1(query []string) (int, error) {
	numberOfZero := 0
	dialPoint := 50

	for _, line := range query {
		numberString := line[1:]
		number, err := strconv.Atoi(numberString)
		if err != nil {
			return 0, err
		}

		if line[0] == 'L' {
			dialPoint -= number
		} else if line[0] == 'R' {
			dialPoint += number
		} else {
			return 0, errors.New("Invalid direction: " + line + " must start with either L or R")
		}

		for 0 > dialPoint || dialPoint > 99 {
			if dialPoint > 99 {
				dialPoint -= 100
			}
			if dialPoint < 0 {
				dialPoint += 100
			}
		}

		if dialPoint == 0 {
			numberOfZero++
		}
	}

	return numberOfZero, nil
}

func part2(query []string) (int, error) {
	numberOfZero := 0
	dialPoint := 50
	previousNumber := dialPoint

	for _, line := range query {
		previousNumber = dialPoint
		numberString := line[1:]
		number, err := strconv.Atoi(numberString)
		if err != nil {
			return 0, err
		}

		if line[0] == 'L' {
			numberOfZero += number / 100
			dialPoint -= number % 100
		} else if line[0] == 'R' {
			numberOfZero += number / 100
			dialPoint += number % 100
		} else {
			return 0, errors.New("Invalid direction: " + line + " must start with either L or R")
		}

		if dialPoint > 99 {
			dialPoint -= 100

			if dialPoint != 0 && previousNumber != 0 {
				numberOfZero++
			}
		} else if dialPoint < 0 {
			dialPoint += 100

			if dialPoint != 0 && previousNumber != 0 {
				numberOfZero++
			}
		}

		if dialPoint == 0 {
			numberOfZero++
		}
	}

	return numberOfZero, nil
}
