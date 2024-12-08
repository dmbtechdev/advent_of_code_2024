pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

mod utils;
pub use utils::*;

#[cfg(test)]
mod tests_day1 {
    use super::*;

    #[test]
    fn day1_test_part1_with_correct_result() {
        let result = day1::part1("./data/day1_test_data");
        assert_eq!(result, Solution::Integer32(11));
    }

    #[test]
    fn day1_test_part1_with_wrong_result() {
        let result = day1::part1("./data/day1_test_data");
        assert_ne!(result, Solution::Integer32(12));
    }

    #[test]
    fn day1_test_part2_with_correct_result() {
        let result = day1::part2("./data/day1_test_data");
        assert_eq!(result, Solution::Integer32(31));
    }

    #[test]
    fn day1_test_part2_with_wrong_result() {
        let result = day1::part2("./data/day1_test_data");
        assert_ne!(result, Solution::Integer32(33));
    }

}

#[cfg(test)]
mod tests_day2 {
    use super::*;

    #[test]
    fn day2_test_part1_with_correct_result() {
        let result = day2::part1("./data/day2_test_data");
        assert_eq!(result, Solution::Integer32(2));
    }

    #[test]
    fn day2_test_part1_with_wrong_result() {
        let result = day2::part1("./data/day2_test_data");
        assert_ne!(result, Solution::Integer32(3));
    }

    #[test]
    fn day2_test_part2_with_correct_result() {
        let result = day2::part2("./data/day2_test_data");
        assert_eq!(result, Solution::Integer32(4));
    }

    #[test]
    fn day2_test_part2_with_wrong_result() {
        let result = day2::part2("./data/day2_test_data");
        assert_ne!(result, Solution::Integer32(5));
    }

}


#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn day3_test_part1_with_correct_result() {
        let result = day3::part1("./data/day3_part1_test_data");
        assert_eq!(result, Solution::Integer32(161));
    }

    #[test]
    fn day3_test_part1_with_wrong_result() {
        let result = day3::part1("./data/day3_part1_test_data");
        assert_ne!(result, Solution::Integer32(151));
    }

    #[test]
    fn day3_test_part2_with_correct_result() {
        let result = day3::part2("./data/day3_part2_test_data");
        assert_eq!(result, Solution::Integer32(48));
    }

    #[test]
    fn day3_test_part2_with_wrong_result() {
        let result = day3::part2("./data/day3_part2_test_data");
        assert_ne!(result, Solution::Integer32(50));
    }

}

#[cfg(test)]
mod tests_day4 {
    use super::*;

    #[test]
    fn day4_test_part1_with_correct_result() {
        let result = day4::part1("./data/day4_test_data");
        assert_eq!(result, Solution::Integer32(18));
    }

    #[test]
    fn day4_test_part1_with_wrong_result() {
        let result = day4::part1("./data/day4_test_data");
        assert_ne!(result, Solution::Integer32(19));
    }

    #[test]
    fn day4_test_part2_with_correct_result() {
        let result = day4::part2("./data/day4_test_data");
        assert_eq!(result, Solution::Integer32(9));
    }

    #[test]
    fn day4_test_part2_with_wrong_result() {
        let result = day4::part2("./data/day4_test_data");
        assert_ne!(result, Solution::Integer32(10));
    }

}

#[cfg(test)]
mod tests_day5 {
    use super::*;

    #[test]
    fn day5_test_part1_with_correct_result() {
        let result = day5::part1("./data/day5_test_data");
        assert_eq!(result, Solution::Integer32(143));
    }

    #[test]
    fn day5_test_part1_with_wrong_result() {
        let result = day5::part1("./data/day5_test_data");
        assert_ne!(result, Solution::Integer32(144));
    }

    #[test]
    fn day5_test_part2_with_correct_result() {
        let result = day5::part2("./data/day5_test_data");
        assert_eq!(result, Solution::Integer32(123));
    }

    #[test]
    fn day5_test_part2_with_wrong_result() {
        let result = day5::part2("./data/day5_test_data");
        assert_ne!(result, Solution::Integer32(124));
    }

}