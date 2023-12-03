use day01::add;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d)").unwrap();
}

fn main() {
    let input = include_str!("../input_part1.txt").to_string();

    let numbers = process(input);
    let total = numbers.iter().fold(0, |acc, val| acc + val);

    println!("Total: {}", total);
}

fn process(input: String) -> Vec<i32> {
    input.lines().map(|line| {
        println!("Line: {}", line);

        // Gather all integer matches, convert into `String` and collect into `Vec`
        let matches = RE.find_iter(line)
            .map(|m| m.as_str().to_owned())
            .collect::<Vec<String>>();

        let a = matches.first().unwrap().as_str();
        let b = matches.last().unwrap().as_str();

        let res = a.to_string() + b;

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
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();

        let numbers = process(input);
        let total = numbers.iter().fold(0, |acc, val| acc + val);

        assert_eq!(numbers.len(), 4);
        assert_eq!(total, 142);
        itertools::assert_equal(&numbers, [12i32, 38i32, 15i32, 77i32].iter());
    }
}
