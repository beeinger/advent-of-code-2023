use std::fs;

use rayon::prelude::*;

pub struct SetOfCubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

pub fn sum_possible_ids(input: &str) -> u32 {
    input
        .split('\n')
        .par_bridge()
        .map(|line| {
            let mut split = line.split("Game ").nth(1).unwrap().split(": ");
            let game_id = split.next().unwrap().parse::<u32>().unwrap();
            let is_possible = split
                .next()
                .unwrap()
                .split("; ")
                .par_bridge()
                .map(|set| {
                    set.split(", ").fold(
                        SetOfCubes {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut acc, cube| {
                            let mut split = cube.split(' ');
                            let count = split.next().unwrap().parse::<u32>().unwrap();
                            let color = split.next().unwrap();

                            match color {
                                "red" => acc.red += count,
                                "green" => acc.green += count,
                                "blue" => acc.blue += count,
                                _ => (),
                            }
                            acc
                        },
                    )
                })
                .fold(
                    || true,
                    |acc, s| s.red <= RED_MAX && s.green <= GREEN_MAX && s.blue <= BLUE_MAX && acc,
                )
                .all(|cond| cond);

            if is_possible {
                game_id
            } else {
                0
            }
        })
        .sum()
}

pub fn get_input() -> String {
    fs::read_to_string("./src/day_2/input.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test() {
        assert_eq!(sum_possible_ids(TEST), 8);
    }

    #[test]
    fn test_get_input() {
        assert_eq!(get_input().len(), 10689);
    }
}
