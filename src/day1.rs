use crate::utils::{Solution,read_as_vector_of_string};

pub fn part1(file_path:&str) -> Solution {

    let (mut list1,mut list2):(Vec<i32>,Vec<i32>) = read_as_vector_of_string(file_path)
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
        })
        .unzip();

    list1.sort();
    list2.sort();

    let mut difference = 0;

    for (left, right) in list1.iter().zip(list2.iter()) {
        difference += (left - right).abs();
    };

    difference.into()
}

pub fn part2(file_path:&str) -> Solution {
    
    let (list1,list2):(Vec<i32>,Vec<i32>) = read_as_vector_of_string(file_path)
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
        })
        .unzip();

    let mut similarity = 0;

    for list1_item in &list1 {
        for list2_item in &list2 {
            if list1_item == list2_item {
                similarity += list1_item;
            }
        }
    }

    similarity.into()
}