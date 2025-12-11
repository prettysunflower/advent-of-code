use std::ops::RangeInclusive;

fn get_bounds(range: &str) -> (usize, usize) {
    let (lower_bound_str, upper_bound_str) = range.split_once("-").expect("invalid range string");
    (
        lower_bound_str.parse::<usize>().unwrap_or_else(|_| panic!("{} is not a valid number", lower_bound_str)),
        upper_bound_str.parse::<usize>().unwrap_or_else(|_| panic!("{} is not a valid number", lower_bound_str))
    )
}

pub fn puzzle_process_input<'a>(query: &'a Vec<&str>) -> Result<(&'a[&'a str], Vec<RangeInclusive<usize>>), &'static str> {
    let empty_input_index = query.iter().position(|&x| x.is_empty());
    if empty_input_index.is_none() {
        return Err("empty input index not found");
    }

    let (ranges_str, values_to_check) = query.split_at(empty_input_index.unwrap());

    let mut ranges = Vec::new();

    for range in ranges_str {
        let (lower_bound, upper_bound) = get_bounds(range);
        ranges.push(lower_bound..=upper_bound);
    }

    Ok((values_to_check, ranges))
}

pub fn puzzle_part1(query: &Vec<&str>) -> Result<usize, &'static str> {
    let (values_to_check, ranges) = puzzle_process_input(query)?;

    let mut number_of_fresh_ingredients = 0usize;

    'values_loop: for value_to_check_str in values_to_check {
        if value_to_check_str.is_empty() {
            continue;
        }

        let value_to_check = value_to_check_str.parse::<usize>().expect("invalid value to check");
        for range in &ranges {
            if range.contains(&value_to_check) {
                number_of_fresh_ingredients += 1;
                continue 'values_loop;
            }
        }
    }

    Ok(number_of_fresh_ingredients)
}

pub fn puzzle_part2(query: &Vec<&str>) -> Result<usize, &'static str> {
    let (_, mut ranges) = puzzle_process_input(query)?;

    // Taking the number of all ranges and checking one-by-one if each of them are unique
    // would be _way_ too computationally expensive and would take too much time.
    // So, here, we're gonna simplify the ranges by merging the overlapping ones.

    // And, to start, we're gonna first sort the ranges ascendingly

    ranges.sort_by(|range1, range2|
        range1.start().cmp(range2.start()));

    // Let's initialize some variables for the loop...

    let mut new_ranges = Vec::new();
    let mut index = 0;
    let mut start = None;
    let mut end = None;

    loop {
        if index == ranges.len() {
            // Break the loop if we exhausted all possible ranges
            if let Some(_start) = start && let Some(_end) = end {
                new_ranges.push(_start..=_end);
            }
            break;
        }

        // Are we at the start where start and end are None? Let's initialize those then.

        if start.is_none() || end.is_none() {
            start = Some(*ranges[index].start());
            end = Some(*ranges[index].end());
            index += 1;

            continue;
        }

        // What we're doing here is merging the ranges.
        // Since we have sorted the ranges by their start earlier, we know that the start ranges are
        // going in ascending order.
        // So, if the start of the currently analyzed range is lower than the end of the previous range,
        // we can know that those two ranges intersect, and we can merge them by taking the start of
        // the previous range, and the end of the current range.
        // Example: 0..=5 and 3..=7 can be simplified to 0..=7, since 3 < 5.

        let _start = start.expect("Start is None after is_none, somehow");
        let _end = end.expect("End is None after is_none, somehow");
        let range = &ranges[index];
        
        if range.start() <= &_end {
            if _end < *range.end() {
                end = Some(*range.end());
            }
            index += 1;
            continue;
        } else {
            new_ranges.push(_start..=_end);
            start = Some(*range.start());
            end = Some(*range.end());
            index += 1;
        }
    }

    // At this point, none of the ranges should be overlapping, we can calculate how much unique
    // ingredients there are by simply making the sum of end - start + 1 of each ranges.

    let sum_of_numbers = new_ranges.iter().fold(0usize, |acc, range| {
        acc + (range.end() - range.start()) + 1
    });

    Ok(sum_of_numbers)
}