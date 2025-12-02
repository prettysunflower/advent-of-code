pub fn puzzle(query: Vec<&str>) -> i32 {
    let mut number_of_zero = 0;
    let mut dial_point: i32 = 50;

    for line in query.iter() {
        println!("{}", line);
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

        println!("{}", dial_point);
    }

    number_of_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let result = puzzle(vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82"
        ]);
        assert_eq!(result, 3);
    }
}
