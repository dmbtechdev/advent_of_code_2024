use advent_of_code_2024::*;
use colored::Colorize;
use std::{env, process};
use std::time::Instant;

fn main() {
    // print!("{}[2J", 27 as char);
    std::process::Command::new("clear").status().unwrap();

    let args: Vec<String> = env::args().collect();
    
    let (mut day,mut part) = (1,1);

    if args.len() > 2 {
        day = args[1].parse().expect("Day number is expected");
        part = args[2].parse().expect("Part number is expected");
    } else {
        println!("\nYou may enter 'cargo run [day#] [part#]\t Example: cargo run 2 2\n\n")
    }

    let now = Instant::now();

    let solution = match (day, part) {
        (1, 1) => day1::part1("./data/day1_data"),
        (1, 2) => day1::part2("./data/day1_data"),
        (2, 1) => day2::part1("./data/day2_data"),
        (2, 2) => day2::part2("./data/day2_data"),
        (3, 1) => day3::part1("./data/day3_data"),
        (3, 2) => day3::part2("./data/day3_data"),
        (4, 1) => day4::part1("./data/day4_data"),
        (4, 2) => day4::part2("./data/day4_data"),
        (5, 1) => day5::part1("./data/day5_data"),
        (5, 2) => day5::part2("./data/day5_data"),
        (d , p) if d >=5 && p>=3 => {println!("Invalid day or part. Max day is 5. Max part is 2.");process::exit(1)},
        (d , p) if d <5 && p>2 => {println!("Invalid part. Max part is 2.");process::exit(1)},
        (d , p) if d >=5 && (p == 1 || p == 2) => {println!("Invalid day. Max day is 5.");process::exit(1)},
        _ => {println!("Invalid day or part. Max day is 5. Max part is 2.");process::exit(1)}
    }.to_string().green();

    let elapsed = now.elapsed().as_micros();
    let html = format!("{}", (match day {
        1 => "https://adventofcode.com/2024/day/1",
        2 => "https://adventofcode.com/2024/day/2",
        3 => "https://adventofcode.com/2024/day/3",
        4 => "https://adventofcode.com/2024/day/4",
        5 => "https://adventofcode.com/2024/day/5",
        _ => ""
    }.blue()));

    println!(
        " Advant of Code 2024:\n Day {} Part {}\n\n HTTP Link: {}\n Solution {}\n Time Taken: {} microsec\n\n\n",
        day, part, html, solution, elapsed
    );
    
}