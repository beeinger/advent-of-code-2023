use rayon::prelude::*;

use super::part_1::SetOfCubes;

pub fn sum_powers_of_min_sets(input: &str) -> u32 {
    input
        .split('\n')
        .par_bridge()
        .map(|line| {
            let min_set = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .par_bridge()
                .map(|sets| {
                    sets.split(", ").fold(
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
                .reduce(
                    || SetOfCubes {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |acc, s| SetOfCubes {
                        red: acc.red.max(s.red),
                        green: acc.green.max(s.green),
                        blue: acc.blue.max(s.blue),
                    },
                );

            min_set.red * min_set.green * min_set.blue
        })
        .sum()
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
        assert_eq!(sum_powers_of_min_sets(TEST), 2286);
    }
}
