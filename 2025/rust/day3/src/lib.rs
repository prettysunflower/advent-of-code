pub fn puzzle_part1(query: &Vec<&str>) -> usize {
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

        let mut max = 0;

        for (index_a, battery_a) in batteries.iter().enumerate() {
            for (index_b, battery_b) in batteries.iter().enumerate() {
                if index_a >= index_b {
                    continue;
                }

                let value = battery_a * 10 + battery_b;
                if value > max {
                    max = value;
                }
            }
        }

        println!("Max value is {}", max);
        sum += max;
    }

    sum
}