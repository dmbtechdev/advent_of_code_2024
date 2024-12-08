use crate::utils::{Solution,read_to_string};
use regex::Regex;

pub fn part1(file_path: &str) -> Solution {
        
    let buffer = read_to_string(file_path);
    // Each d will be captured 
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut multiplications_total = 0;

    // each cap will have cap[0],cap[1] and cap[2] where cap[1] is the first d and cap[2] is the second d from regex
    // cap[0] is the the overall
    // println!("{:?}", cap);
    for cap in pattern.captures_iter(buffer.as_str()) {
        let num1 = cap[1].parse::<i32>().unwrap();
        let num2 = cap[2].parse::<i32>().unwrap();
        multiplications_total += num1 * num2;
    }

    multiplications_total.into()
}

pub fn part2(file_path: &str) -> Solution {
        
    let buffer = read_to_string(file_path);
    // "do", "don" and both "d"s will be captured.
    // cap[0] is the the overall
    // println!("{:?}", cap);
    let pattern = Regex::new(r"(do)\(\)|(don)\'t\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut multiplications_total = 0;
    let mut do_lines = true;

    // each cap will have cap[0], cap[1], cap[2] and cap[3] where cap[2] is the first d and cap[3] is the second d from regex
    for cap in pattern.captures_iter(buffer.as_str()) {
        if cap.get(1).is_some() {do_lines = true;}
        else if cap.get(2).is_some() {do_lines = false;}
        else if do_lines {
            let num1 = cap[3].parse::<i32>().unwrap();
            let num2 = cap[4].parse::<i32>().unwrap();
            multiplications_total += num1 * num2;
        }
    }

    multiplications_total.into()
}