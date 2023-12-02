use rayon::prelude::*;
use regex::Regex;

pub fn sum_calibration(input: &str) -> i32 {
    input
        .split('\n')
        .par_bridge()
        .map(|line| {
            let re =
                Regex::new(r"^(?:\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

            let digits: Vec<_> = line
                .par_char_indices()
                .fold(Vec::new, |mut r, (i, _)| {
                    if let Some(mat) = re.find(&line[i..]) {
                        r.push(mat.as_str());
                    };
                    r
                })
                .flatten()
                .map(|digit| match digit {
                    "zero" => "0",
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => digit,
                })
                .collect();

            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test() {
        assert_eq!(sum_calibration(TEST), 281);
    }

    #[test]
    fn regex() {
        let re = Regex::new(r"^(?:\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let input = "abcone2threexyz";
        let mut result = Vec::new();

        for (i, _) in input.char_indices() {
            if let Some(mat) = re.find(&input[i..]) {
                result.push(mat.as_str().to_string());
            }
        }

        assert_eq!(result, vec!["one", "2", "three"]);
    }
}
