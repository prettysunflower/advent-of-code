pub fn puzzle(query: Vec<String>) -> (usize, Vec<String>) {
    let mut sum_of_accessible_rolls = 0;
    let mut output_string: Vec<String> = Vec::new();

    for (y_position, line) in query.iter().enumerate() {
        let mut output_line: Vec<&str> = Vec::new();

        for (x_position, character) in line.chars().enumerate() {
            if character != '@' {
                output_line.push(".");
                continue;
            }

            let mut total_of_surrounding_rolls = 0;

            for x_diff in -1..=1isize {
                for y_diff in -1..=1isize {
                    if x_diff == 0 && y_diff == 0 { continue; }

                    let x_check = (x_position as isize) + x_diff;
                    let y_check = (y_position as isize) + y_diff;

                    if y_check < 0 && x_check < 0 {
                        // If it was out of bounds on top or left sides
                        continue;
                    }

                    let x_check_usize = x_check as usize;
                    let y_check_usize = y_check as usize;

                    if y_check_usize >= query.len() || x_check_usize >= line.len() {
                        // If it was out of bounds on bottom or right sides
                        continue;
                    }

                    if query[y_check_usize].chars().collect::<Vec<char>>()[x_check_usize] == '@' {
                        total_of_surrounding_rolls += 1;
                    }
                }
            }

            if total_of_surrounding_rolls < 4 {
                sum_of_accessible_rolls += 1;
                output_line.push("X");
            } else {
                output_line.push("@");
            }
        }
        output_string.push(output_line.join(""))
    }

    (sum_of_accessible_rolls, output_string)
}

pub fn puzzle_part2(query: Vec<String>) -> (usize, Vec<String>) {
    let mut sum_of_accessible_rolls = 0;
    let mut output_string: Vec<String> = query.clone();

    loop {
        let (removed_rolls, _output_string) = puzzle(output_string);
        output_string = _output_string;
        sum_of_accessible_rolls += removed_rolls;

        if removed_rolls == 0 {
            break;
        }
    }

    (sum_of_accessible_rolls, output_string)
}