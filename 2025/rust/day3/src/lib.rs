fn calculate_vec_value(usize_vec: &[usize]) -> usize {
    let mut sum = 0;
    for (index, digit) in usize_vec.iter().rev().enumerate() {
        sum += digit * 10usize.pow(index as u32);
    }
    sum
}

fn find_best_combination(batteries: Vec<usize>, limit: usize) -> usize {
    let mut best_combination = vec![];

    for battery in batteries {
        if best_combination.len() < limit {
            best_combination.push(battery);
            continue;
        }

        let mut best_value = calculate_vec_value(&best_combination);
        let mut temp_best_combination = vec![];

        for i in 0..best_combination.len() {
            let mut working_vec = best_combination.clone();
            working_vec.push(battery);
            working_vec.remove(i);
            let new_value = calculate_vec_value(&working_vec);
            if new_value > best_value {
                temp_best_combination = working_vec;
                best_value = new_value;
            }
        }

        if !temp_best_combination.is_empty() {
            best_combination = temp_best_combination;
        }
    }

    println!("Best combination: {:?}", best_combination);
    println!("Best value: {:?}", calculate_vec_value(&best_combination));
    calculate_vec_value(&best_combination)
}

pub fn puzzle(query: &Vec<&str>, limit: usize) -> usize {
    let mut sum = 0;
    for line in query {
        println!("Line: {}", line);
        let batteries: Vec<usize> = line
            .chars()
            .map(|char|
                char
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_| panic!("{} is not a number", char))
            )
            .collect();

        sum += find_best_combination(batteries, limit);
    }

    sum
}