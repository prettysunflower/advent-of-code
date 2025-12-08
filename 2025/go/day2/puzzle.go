package day2

import (
	"errors"
	"strconv"
	"strings"
)

func getBounds(stringRange string) (int, int, []error) {
	splitString := strings.Split(stringRange, "-")
	var errs []error

	if len(splitString) != 2 {
		return 0, 0, []error{errors.New("invalid range format")}
	}

	lowerBound, err := strconv.Atoi(splitString[0])
	if err != nil {
		errs = append(errs, err)
	}
	upperBound, err := strconv.Atoi(splitString[1])
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return 0, 0, errs
	}

	return lowerBound, upperBound, errs
}

func part1(input string) (int, []error) {
	ranges := strings.Split(input, ",")
	sum := 0
	var errs []error

	for _, rangeString := range ranges {
		lowerBound, upperBound, rangeErrs := getBounds(rangeString)
		if len(errs) > 0 {
			errs = append(errs, rangeErrs...)
			continue
		}

		for i := lowerBound; i <= upperBound; i++ {
			strNum := strconv.Itoa(i)
			x, y := strNum[:len(strNum)/2], strNum[len(strNum)/2:]
			if x == y {
				sum += i
			}
		}
	}

	if len(errs) > 0 {
		return 0, errs
	}

	return sum, nil
}

func part2(input string) (int, []error) {
	ranges := strings.Split(input, ",")
	sum := 0
	var errs []error

	for _, rangeString := range ranges {
		lowerBound, upperBound, rangeErrs := getBounds(rangeString)
		if len(errs) > 0 {
			errs = append(errs, rangeErrs...)
			continue
		}

	numberLoop:
		for num := lowerBound; num <= upperBound; num++ {
			strNum := strconv.Itoa(num)

			for captureSize := 1; captureSize <= len(strNum)/2; captureSize++ {
				if len(strNum)%captureSize != 0 {
					continue
				}

				numberOfRepeat := len(strNum) / captureSize
				repeatStr := strings.Repeat(strNum[0:captureSize], numberOfRepeat)
				if strNum == repeatStr {
					sum += num
					continue numberLoop
				}
			}
		}
	}

	if len(errs) > 0 {
		return 0, errs
	}

	return sum, nil
}
