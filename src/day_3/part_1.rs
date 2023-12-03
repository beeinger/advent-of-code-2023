use std::{
    fs,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

pub struct Number {
    pub value: Vec<char>,
    /// (\[x_start, x_end], y)
    pub coords: ([u32; 2], u32),
}

impl Number {
    pub fn new(value: Vec<&char>, coords: (u32, u32)) -> Self {
        let length = value.len() as u32;
        Self {
            value: value.into_iter().copied().collect(),
            coords: ([coords.0, coords.0 + (length - 1)], coords.1),
        }
    }

    pub fn get_numeric_value(&self) -> u32 {
        self.value
            .iter()
            .fold(String::new(), |mut acc, char| {
                acc.push(*char);
                acc
            })
            .parse::<u32>()
            .unwrap()
    }

    pub fn get_adjacent_coords(&self, max_x: u32, max_y: u32) -> Vec<(u32, u32)> {
        let mut coords = Vec::new();

        for y in self.coords.1 as i32 - 1..=self.coords.1 as i32 + 1 {
            for x in self.coords.0[0] as i32 - 1..=self.coords.0[1] as i32 + 1 {
                if y < 0 || x < 0 {
                    continue;
                };

                let x = x as u32;
                let y = y as u32;

                if (x > max_x || y > max_y)
                    || (y == self.coords.1 && (self.coords.0[0] <= x && x <= self.coords.0[1]))
                {
                    continue;
                }

                coords.push((x, y));
            }
        }

        coords
    }
}

pub fn sum_park_nums(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input
        .par_split('\n')
        .into_par_iter()
        .map(|line| line.par_chars().into_par_iter().collect::<Vec<char>>())
        .collect();
    let max_x = input[0].len() as u32 - 1;
    let max_y = input.len() as u32 - 1;

    let part_numbers = Arc::new(Mutex::new(Vec::new()));

    input.par_iter().enumerate().for_each(|(y, line)| {
        let mut chars = line.iter().enumerate();

        while let Some((x, char)) = chars.next() {
            if char == &'.' || !char.is_ascii_digit() {
                continue;
            }

            let mut value = vec![char];
            let coords = (x as u32, y as u32);

            for (_, next_char) in chars.by_ref() {
                if next_char == &'.' || !next_char.is_ascii_digit() {
                    break;
                }

                value.push(next_char);
            }

            let number = Number::new(value, coords);
            let adjacent_coords = number.get_adjacent_coords(max_x, max_y);

            let is_part_number = adjacent_coords.par_iter().any(|(x, y)| {
                let char = &input[*y as usize][*x as usize];
                char != &'.' && !char.is_ascii_digit()
            });

            if is_part_number {
                part_numbers.lock().unwrap().push(number);
            }
        }
    });

    let line_sum = part_numbers
        .lock()
        .unwrap()
        .iter()
        .map(|number| number.get_numeric_value())
        .sum();

    line_sum
}

pub fn get_input() -> String {
    fs::read_to_string("./src/day_3/input.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test() {
        assert_eq!(sum_park_nums(TEST), 4361);
    }

    #[test]
    fn test_get_input() {
        assert_eq!(get_input().len(), 19739);
    }
}
