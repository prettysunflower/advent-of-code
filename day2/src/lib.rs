fn get_bounds(range: &str) -> (usize, usize) {
    let (lower_bound_str, upper_bound_str) = range.split_once("-").expect("invalid range string");
    (
        lower_bound_str.parse::<usize>().unwrap_or_else(|_| panic!("{} is not a valid number", lower_bound_str)),
        upper_bound_str.parse::<usize>().unwrap_or_else(|_| panic!("{} is not a valid number", lower_bound_str))
    )
}

pub fn puzzle_part1(input: &str) -> usize {
    let ranges: Vec<&str> = input.split(",").collect();

    let mut sum: usize = 0;

    for range in ranges {
        let (lower_bound, upper_bound) = get_bounds(range);

        for number in lower_bound..=upper_bound {
            let str_num = number.to_string();
            let (x,y) = str_num.split_at(str_num.len() / 2);
            if x == y {
                sum += number;
            }
        }
    }

    sum
}

pub fn puzzle_part2(input: &str) -> usize {
    let ranges: Vec<&str> = input.split(",").collect();

    let mut sum: usize = 0;

    for range in ranges {
        let (lower_bound, upper_bound) = get_bounds(range);

        'number_loop: for number in lower_bound..=upper_bound {
            let str_num = number.to_string();
            for capture_size in 1..=str_num.len() / 2 {
                if (str_num.len() % capture_size) != 0 { continue; }
                let number_of_repeat = str_num.len() / capture_size;
                let repeat_string = &str_num[0..capture_size].repeat(number_of_repeat);
                if str_num == *repeat_string {
                    sum += number;
                    continue 'number_loop;
                }
            }
        }
    }

    sum
}