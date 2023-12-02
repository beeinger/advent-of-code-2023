use std::fs;

use rayon::prelude::*;

pub fn sum_calibration(input: &str) -> i32 {
    input
        .split('\n')
        .par_bridge()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let chars_back = line.chars().rev().collect::<Vec<char>>();
            let mut iterators = vec![chars.iter(), chars_back.iter()];
            let digits: Vec<_> = iterators
                .par_iter_mut()
                .map(|iterator| loop {
                    let next = iterator.next();
                    if next.map(|n| n.is_ascii_digit()).unwrap_or(false) {
                        break next.unwrap();
                    }
                })
                .collect();
            format!("{}{}", digits[0], digits[1])
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

pub fn get_input() -> String {
    fs::read_to_string("./src/day_1/input.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1abc2
pqr3stu8vwx 
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test() {
        assert_eq!(sum_calibration(TEST), 142);
    }

    #[test]
    fn test_get_input() {
        assert_eq!(get_input().len(), 21825);
    }
}
