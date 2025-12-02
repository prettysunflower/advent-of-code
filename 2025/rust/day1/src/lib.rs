pub fn puzzle_part1(query: &Vec<&str>) -> i32 {
    let mut number_of_zero = 0;
    let mut dial_point: i32 = 50;

    for line in query.iter() {
        if let Some(number_str) = line.strip_prefix("L") {
            let number = number_str.parse::<i32>().expect("Invalid number");
            dial_point -= number;
        } else if let Some(number_str) = line.strip_prefix("R") {
            let number = number_str.parse::<i32>().expect("Invalid number");
            dial_point += number;
        } else {
            panic!("Line is starting with an unexpected letter");
        }

        while !(0..=99).contains(&dial_point) {
            if dial_point > 99 {
                dial_point -= 100;
            } else if dial_point < 0 {
                dial_point += 100;
            }
        }

        if dial_point == 0 {
            number_of_zero += 1;
        }
    }

    number_of_zero
}

pub fn puzzle_part2(query: &Vec<&str>) -> i32 {
    let mut number_of_zero = 0;
    let mut dial_point: i32 = 50;
    let mut previous_number;

    for line in query.iter() {
        let number: i32;
        previous_number = dial_point;

        if let Some(number_str) = line.strip_prefix("L") {
            number = number_str.parse::<i32>().expect("Invalid number");
            number_of_zero += number.div_euclid(100);
            dial_point -= number % 100;
        } else if let Some(number_str) = line.strip_prefix("R") {
            number = number_str.parse::<i32>().expect("Invalid number");
            number_of_zero += number.div_euclid(100);
            dial_point += number % 100;
        } else {
            panic!("Line is starting with an unexpected letter");
        }

        if dial_point > 99 {
            dial_point -= 100;

            if dial_point != 0 && previous_number != 0 {
                number_of_zero += 1;
            }
        } else if dial_point < 0 {
            dial_point += 100;
            if dial_point != 0 && previous_number != 0 {
                number_of_zero += 1;
            }
        }

        if dial_point == 0 {
            number_of_zero += 1;
        }
    }

    number_of_zero
}