use day01::add;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d)").unwrap();
}

fn main() {
    let input = include_str!("../input.txt").to_string();

    let numbers = process(input);
    let total = numbers.iter().fold(0, |acc, val| acc + val);

    println!("Total: {}", total);
}

fn process(input: String) -> Vec<i32> {
    let re_map = [
        ("one", "o1e"),  
        ("two", "t2o"),  
        ("three", "t3e"),  
        ("four", "f4r"),  
        ("five", "f5e"),  
        ("six", "s6x"),  
        ("seven", "s7n"),  
        ("eight", "e8t"),  
        ("nine", "n9e"),  
    ];
    
    input.lines().map(|line| {
        let mut text = line.to_owned();
        
        re_map.iter().for_each(|(needle, repl) | {
            text = text.replace(needle, repl);
        });
        
        // Gather all integer matches, convert into `String` and collect into `Vec`
        let matches = RE.find_iter(text.as_str())
            .map(|m| m.as_str().to_owned())
            .collect::<Vec<String>>();

        // Even if there is just one match per line, repeat match twice
        let a = matches.first().unwrap().as_str();
        let b = matches.last().unwrap().as_str();

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
        let total = numbers.iter().fold(0, |acc, val| acc + val);
        
        assert_eq!(numbers.len(), 1);
        assert_eq!(total, 22);
        itertools::assert_equal(&numbers, [22i32].iter());
    }
    
    #[test]
    #[no_mangle]
    fn combined_numbers() {
        let input = "eightwone".to_string();
        
        let numbers = process(input);
        let total = numbers.iter().fold(0, |acc, val| acc + val);

        assert_eq!(numbers.len(), 1);
        assert_eq!(total, 81);
        itertools::assert_equal(&numbers, [81i32].iter());
    }
}
