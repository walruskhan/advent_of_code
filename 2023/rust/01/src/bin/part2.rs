use day01::add;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
}

fn main() {
    let input = include_str!("../input.txt").to_string();

    let numbers = process(input);
    let total = numbers.iter().fold(0, |acc, val| acc + val);

    println!("Total: {}", total);
}

pub fn normalize(val: &str) -> &str {
    match val {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => val
    }
}

fn process(input: String) -> Vec<i32> {
    input.lines().map(|line| {
        // Gather all integer matches, convert into `String` and collect into `Vec`
        let matches = RE.find_iter(line)
            .map(|m| m.as_str().to_owned())
            .collect::<Vec<String>>();

        let a = normalize(matches.first().unwrap().as_str());
        let b = normalize(matches.last().unwrap().as_str());

        let res = a.to_string() + b;

        println!("Line: {} ({} + {} = {})", line, a, b, res);

        res.parse::<i32>().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use log::debug;
    use super::*;

    #[test]
    #[no_mangle]
    fn example() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();

        let numbers = process(input);
        let total = numbers.iter().fold(0, |acc, val| acc + val);

        assert_eq!(numbers.len(), 7);
        assert_eq!(total, 281);
        itertools::assert_equal(&numbers, [29i32, 83i32, 13i32, 24i32, 42i32, 14i32, 76i32].iter());
    }
}
