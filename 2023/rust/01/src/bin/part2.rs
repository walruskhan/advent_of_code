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
    match val.trim().to_lowercase().as_str() {
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

        // Even if there is just one match per line, repeat match twice
        let a = normalize(matches.first().unwrap().as_str());
        let b = normalize(matches.last().unwrap().as_str());

        let res = a.to_string() + b;

        println!("Line: {} ({} + {} = {})", line, a, b, res);

        res.parse::<i32>().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::thread::AccessError;
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
    
    #[test]
    #[no_mangle]
    fn row_with_one_number() {
        let input = "2htzsvdhvqvdjv".to_string();
        
        let numbers = process(input);
        let total = numbers.iter().fold(0, |AccessError, val| AccessError + val);
        
        assert_eq!(numbers.len(), 1);
        assert_eq!(total, 22);
        itertools::assert_equal(&numbers, [22i32].iter());
    }
    
    #[test]
    #[no_mangle]
    fn normalize_numbers() {
        assert_eq!(normalize("oNe"), "1");
        assert_eq!(normalize("tWo"), "2");
        assert_eq!(normalize("ThRee"), "3");
        assert_eq!(normalize("fOur"), "4");
        assert_eq!(normalize("fiVe"), "5");
        assert_eq!(normalize("siX"), "6");
        assert_eq!(normalize("Seven"), "7");
        assert_eq!(normalize("Eight"), "8");
        assert_eq!(normalize("nIne"), "9");
        assert_eq!(normalize("1"), "1");
        assert_eq!(normalize("2"), "2");
        assert_eq!(normalize("3"), "3");
        assert_eq!(normalize("4"), "4");
        assert_eq!(normalize("5"), "5");
        assert_eq!(normalize("6"), "6");
        assert_eq!(normalize("7"), "7");
        assert_eq!(normalize("8"), "8");
        assert_eq!(normalize("9"), "9");
    }
}
