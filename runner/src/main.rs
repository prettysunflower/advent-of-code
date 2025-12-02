use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the day to run the program for
    #[arg(short, long)]
    day: u8,

    /// Input file
    #[arg(short, long)]
    input: String,
}


fn main() {
    let args = Args::parse();

    let day = args.day;
    let input_file = args.input;

    let input_file_content = fs::read_to_string(&input_file).expect("Failed to read input file");

    match day {
        1 => {
            let input: Vec<&str> = input_file_content.split_terminator("\n").collect();
            println!("Part 1: {}", day1::puzzle_part1(&input));
            println!("Part 2: {}", day1::puzzle_part2(&input));
        },
        2 => {
            println!("Part 1: {}", day2::puzzle_part1(input_file_content.as_str()));
            println!("Part 2: {}", day2::puzzle_part2(input_file_content.as_str()));
        }
        _ => println!("Invalid day: {}", day),
    }
}
