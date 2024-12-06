use crate::utils::{Solution,read_lines_as_vec_of_vec_of_chars};

pub fn part1(file_path: &str) -> Solution {
        
    let vectors = read_lines_as_vec_of_vec_of_chars(file_path);
    // println!("{:?} {:?}", vectors[0], vectors[1]);

    let directions = vec![
        (0, 1),   // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),   // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];

    let target = "XMAS".chars().collect::<Vec<char>>();
    let rows = vectors.len();
    let cols = vectors[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in &directions {
                if matches_pattern(&vectors, &target, i as isize, j as isize, *dx, *dy) {
                    count += 1;
                }
            }
        }
    }

    count.into()

}

fn matches_pattern(
    grid: &Vec<Vec<char>>,
    pattern: &Vec<char>,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (k, &c) in pattern.iter().enumerate() {
        let nx = x + k as isize * dx;
        let ny = y + k as isize * dy;

        if nx < 0 || ny < 0 || nx >= rows || ny >= cols || grid[nx as usize][ny as usize] != c {
            return false;
        }
    }

    true
}

pub fn part2(file_path: &str) -> Solution {
    let grid = read_lines_as_vec_of_vec_of_chars(file_path);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Iterate through each potential center cell
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if is_x_mas_pattern(&grid, i, j) {
                count += 1;
            }
        }
    }

    count.into()
}

fn is_x_mas_pattern(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Check if center is 'A'
    if grid[i][j] != 'A' {
        return false;
    }

    // Check diagonal patterns for MAS
    let top_left = grid[i-1][j-1];
    let top_right = grid[i-1][j+1];
    let bottom_left = grid[i+1][j-1];
    let bottom_right = grid[i+1][j+1];

    // Check both diagonals for M and S in any order
    let diagonal1_valid = (top_left == 'M' || top_left == 'S') && 
                         (bottom_right == 'M' || bottom_right == 'S') &&
                         top_left != bottom_right;  // Must be different
    
    let diagonal2_valid = (top_right == 'M' || top_right == 'S') &&
                         (bottom_left == 'M' || bottom_left == 'S') &&
                         top_right != bottom_left;  // Must be different

    // Both diagonals must be valid
    diagonal1_valid && diagonal2_valid
}