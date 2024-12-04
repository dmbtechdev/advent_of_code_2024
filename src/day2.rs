use crate::utils::{Solution, read_lines_as_vec_of_vec_of_i32};


pub fn part1(file_path: &str) -> Solution {
        
    let vectors = read_lines_as_vec_of_vec_of_i32(file_path);
    let mut un_safe = 0;

    for vector in &vectors { 
        if !check_safe(vector) {un_safe +=1}
    }

    (vectors.len() - un_safe).into()
}


pub fn part2(file_path: &str) -> Solution {
            
    let vectors = read_lines_as_vec_of_vec_of_i32(file_path);
    let mut un_safe = 0;
    
    for vector in &vectors { 

        let mut once_safe = check_safe(vector);

        if !once_safe {
            
            // check the vector by taking each index out with the remaining items if it is safe
            for index in 0..vector.len() {
                let mut cloned = vector.clone();
                _ = cloned.remove(index);

                once_safe = check_safe(&cloned);

                if once_safe {break;}
            }
            
            if !once_safe {un_safe += 1;}
        }
    }

    (vectors.len()-un_safe).into()

}

fn check_safe(vector:&Vec<i32>) -> bool {

    let increasing = (vector[1] - vector[0]) > 0;
    let decreasing = (vector[1] - vector[0]) < 0;
    
    for couple in vector.windows(2) {
        if  (couple[1] - couple[0]).abs() > 3 || 
            ((couple[1] - couple[0]) > 0 && decreasing) ||
            ((couple[1] - couple[0]) < 0 && increasing) ||
            (couple[1] - couple[0] == 0)
         
            {return false}
    }
    true


    // Initial part1 code
    // let increasing = (vector[1] - vector[0]) > 0;
    // let decreasing = (vector[1] - vector[0]) < 0;
    // for couple in vector.windows(2) {
    //     if  (couple[1] - couple[0]).abs() > 3 || 
    //         ((couple[1] - couple[0]) > 0 && decreasing) ||
    //         ((couple[1] - couple[0]) < 0 && increasing) ||
    //         (couple[1] - couple[0] == 0)

    //         {not_safe +=1; break}
    // }

}


