use crate::utils::{Solution,read_as_vector_of_string};

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}
use Direction::*;

impl Coordinate {
    fn relocate(self, direction: &Direction, max_x:&usize, max_y:&usize) -> Option<Self> {
        let mut target = self.clone();
        match direction {
            Up if target.y > 0 => 
                target.y -= 1,

            Down if target.y < *max_y => 
                target.y += 1,

            Right if target.x < *max_x => 
                target.x += 1,

            Left if target.x > 0 => 
                target.x -= 1,

            UpRight if target.y > 0 && target.x < *max_x => 
                {
                    target.y -= 1; 
                    target.x += 1
                },
            UpLeft if target.y > 0 && target.x > 0 => 
                {
                    target.y -= 1; 
                    target.x -= 1
                },
            DownRight if target.y < *max_y && target.x < *max_x => 
                {
                    target.y += 1; 
                    target.x += 1
                },
            DownLeft if target.y < *max_y && target.x > 0 => 
                {
                    target.y += 1; 
                    target.x -= 1
                },
            _ => return None
        }
        Some(target)
    }
}


// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....

fn search_xmas_pattern_1(data: &Vec<String>, first_location: Coordinate, max_x:&usize, max_y:&usize, direction: &Direction) -> bool {
    
    let second_location = first_location.relocate(direction, max_x, max_y);
    if let Some(location) = second_location {
        if data[location.y].chars().nth(location.x).unwrap() == 'M' {
            let third_location = location.relocate(direction, max_x, max_y);
            if let Some(location) = third_location {
                if data[location.y].chars().nth(location.x).unwrap() == 'A' {
                    let fourth_location = location.relocate(direction, max_x, max_y);
                    if let Some(location) = fourth_location {
                        if data[location.y].chars().nth(location.x).unwrap() == 'S' {
                                return true
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn part1(file_path: &str) -> Solution {
    let data = read_as_vector_of_string(file_path);
    let max_x = data[0].len()-1;
    let max_y = data.len()-1;
    let mut count = 0;
    
    for x in 0..=max_x {
        for y in 0..=max_y {
            
            if data[y].chars().nth(x).unwrap() != 'X' { continue; }
            
            let first_location = Coordinate {
                x,
                y,
            };
            
            let directions = vec![Up, Down, Right, Left, UpRight, UpLeft, DownRight, DownLeft];

            for direction in directions {
                if search_xmas_pattern_1(&data, first_location.clone(), &max_x, &max_y, &direction) { count += 1}
            }
        }
    }
    count.into()

}



// M.S
// .A.
// M.S

// M.M
// .A.
// S.S

// S.M
// .A.
// S.M

// S.S
// .A.
// M.M



fn search_xmas_pattern_2(data: &Vec<String>, first_location: Coordinate, max_x:&usize, max_y:&usize) -> bool {
    
    let directions = vec![vec![UpRight, DownLeft], vec![UpLeft, DownRight]];
    
    let second_location = first_location.clone().relocate(&directions[0][0], max_x, max_y);
    if let Some(location1) = second_location {
        let letter1 = data[location1.y].chars().nth(location1.x).unwrap();
        if letter1 == 'A' || letter1 == 'X' { return false; }
        let third_location = first_location.clone().relocate(&directions[0][1], max_x, max_y);
        if let Some(location2) = third_location {
            let fourth_location = first_location.clone().relocate(&directions[1][0], max_x, max_y);
            let letter2 = data[location2.y].chars().nth(location2.x).unwrap();
            if letter2 == 'A' || letter1 == 'X' { return false; }
            if let Some(location3) = fourth_location {
                let fifth_location = first_location.clone().relocate(&directions[1][1], max_x, max_y);
                let letter3 = data[location3.y].chars().nth(location3.x).unwrap();
                if letter3 == 'A' || letter1 == 'X' { return false; }
                if let Some(location4) = fifth_location {
                    let letter4 = data[location4.y].chars().nth(location4.x).unwrap();
                    if letter4 == 'A' || letter1 == 'X' { return false; }
                    if (letter1 == 'M' && letter2 == 'S' && letter3 == 'S' && letter4 == 'M') 
                    || (letter1 == 'S' && letter2 == 'M' && letter3 == 'M' && letter4 == 'S')
                    || (letter1 == 'M' && letter2 == 'S' && letter3 == 'M' && letter4 == 'S')
                    || (letter1 == 'S' && letter2 == 'M' && letter3 == 'S' && letter4 == 'M') {
                        return true
                    }
                }
            }
        }  
    }
    false
}


pub fn part2(file_path: &str) -> Solution {
    let data = read_as_vector_of_string(file_path);
    let max_x = data[0].len()-1;
    let max_y = data.len()-1;
    let mut count = 0;
    
    for x in 0..=max_x {
        for y in 0..=max_y {
            
            if data[y].chars().nth(x).unwrap() != 'A' { continue; }
            
            let first_location = Coordinate {
                x,
                y,
            };
            
            if search_xmas_pattern_2(&data, first_location, &max_x, &max_y) { count += 1}
        }
    }
    count.into()
}