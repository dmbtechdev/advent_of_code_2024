use std::{fmt::Display, fs, io::{BufRead,Read}};

pub fn read_as_vector_of_string(file_path: &str) -> Vec<String> {
    let file = std::fs::File::open(file_path).unwrap();

    let vector = std::io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    
    vector
}

pub fn read_lines_as_vec_of_vec_of_i32(file_path: &str) -> Vec<Vec<i32>> {
    
    let vectors = read_as_vector_of_string(file_path)
        .into_iter()
        .map(|line|{
            line.split_whitespace()
            .map(|num|num.parse().unwrap())
            .collect()
        })
        .collect::<Vec<Vec<i32>>>();
    
    vectors
}

pub fn read_lines_as_vec_of_vec_of_chars(file_path: &str) -> Vec<Vec<char>> {
    
    let vectors = read_as_vector_of_string(file_path)
        .iter()
        .map(|line|{
            line.chars().collect()            
        })
        .collect::<Vec<Vec<char>>>();
    
    vectors
}

pub fn read_to_string(file_path: &str) -> String {
    let file = fs::File::open(file_path).unwrap();
    let mut buffer = String::new();

    std::io::BufReader::new(file)
        .read_to_string(&mut buffer)
        .unwrap();
    
    buffer
}

#[derive(Debug, PartialEq)]
pub enum Solution {
    Integer32(i32),
}
use Solution::*;

impl From<usize> for Solution {
    fn from(i: usize) -> Self {
        Integer32(i as i32)
    }
}

impl From<i32> for Solution {
    fn from(i: i32) -> Self {
        Integer32(i)
    }
}


impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer32(i) => write!(f, "(as Integer32) is: {}", i)
        }
    }
}