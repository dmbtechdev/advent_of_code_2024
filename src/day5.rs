use crate::utils::{Solution,read_as_vector_of_string};

pub fn part1(file_path: &str) -> Solution {
    let lines = read_as_vector_of_string(file_path);

    let mut order: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    
    let mut section1 = true;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            section1 = false;
            continue;
        } else if section1 {
            let (page_no_1, page_no_2) = line.split_once("|").unwrap();
            order.push((page_no_1.parse::<i32>().unwrap(), page_no_2.parse::<i32>().unwrap()));
        } else {
            updates.push(line.split(",").into_iter().map(|page_no| page_no.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }
    }
    order.sort_unstable();

    for update in updates {
        let mut safe = true;
        let length = update.len();

        'inner: for i in 0..length-1 {
            if order.contains(&(update[i], update[i+1])) {
                continue;
            } else {
                safe = false;
                break 'inner;
            }
        }

        if safe {
            sum += update[(length / 2) as usize];
        } else {
            continue;
        }
    }
    sum.into()

}

pub fn part2(file_path: &str) -> Solution {

    let lines = read_as_vector_of_string(file_path);

    let mut order: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    
    let mut section1 = true;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            section1 = false;
            continue;
        } else if section1 {
            let (page_no_1, page_no_2) = line.split_once("|").unwrap();
            order.push((page_no_1.parse::<i32>().unwrap(), page_no_2.parse::<i32>().unwrap()));
        } else {
            updates.push(line.split(",").into_iter().map(|page_no| page_no.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }
    }

    order.sort_unstable();

    for update in updates {
        let mut vector= update.clone();        
        let length = update.len();
        let mut safe = true;
        
        for i in 0..length-1 {

            if order.contains(&(vector[i], vector[i+1])) {
                continue;
            } else {
                // safe = false;
                vector[i] = update[i+1];
                vector[i+1] = update[i];

                'inner: loop {
                    if let Some(i) = re_order(&vector,&order) {
                        let temp = vector[i];
                        vector[i] = vector[i+1];
                        vector[i+1] = temp;
                    } else {
                        sum += vector[(length / 2) as usize];
                        safe = true;
                        break 'inner;
                    }
                }
            }
        }
        
        if safe {
            continue;
        }
    }
    sum.into()

}

fn re_order(vector: &Vec<i32>, order: &Vec<(i32, i32)>) -> Option<usize> {
    
    let length = vector.len();
    
    for i in 0..length-1 {
        let found = order.contains(&(vector[i], vector[i+1]));
        if found {
            continue;
        } else {
            return Some(i);
        }
    }
    None
}