use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rayon::prelude::*;

use super::part_1::Coords;

type PotentialGears = Arc<RwLock<HashMap<(u32, u32), Vec<Coords>>>>;

pub fn sum_gear_ratios(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input
        .par_split('\n')
        .into_par_iter()
        .map(|line| line.par_chars().into_par_iter().collect::<Vec<char>>())
        .collect();
    let max_x = input[0].len() as u32 - 1;
    let max_y = input.len() as u32 - 1;

    let potential_gears: PotentialGears = Arc::new(RwLock::new(HashMap::new()));

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

            let number = Coords::new(value, coords);
            let adjacent_coords = number.get_adjacent_coords(max_x, max_y);

            adjacent_coords
                .par_iter()
                .filter(|(x, y)| {
                    let char = &input[*y as usize][*x as usize];
                    char == &'*'
                })
                .for_each(|coords| {
                    let mut locked = potential_gears.write().unwrap();
                    let mut prev = locked.get(coords).unwrap_or(&vec![]).clone();
                    prev.push(number.clone());
                    locked.insert(*coords, prev);
                });
        }
    });

    let potential_gears = potential_gears.read().unwrap();
    potential_gears
        .par_iter()
        .fold(
            || 0,
            |acc, (_, numbers)| {
                if numbers.len() == 2 {
                    return acc + (numbers[0].get_numeric_value() * numbers[1].get_numeric_value());
                }
                acc
            },
        )
        .sum()
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
        assert_eq!(sum_gear_ratios(TEST), 467835);
    }
}
