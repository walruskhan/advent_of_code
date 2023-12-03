use std::collections::HashMap;
use lazy_static::lazy_static;
use day02::add;
use regex::{Captures, Regex};

fn main() {
    let input = include_str!("../input.txt");

    let total = input
        .lines()
        .map(|line| parseInput(line))
        .map(|data| get_power(data))
        .sum::<i32>();

    println!("Total: {}", total);
}

// macro for declaring lazily evaluated statics
// https://docs.rs/lazy_static/latest/lazy_static/
// allow static variables that requires computation at runtime
lazy_static! {
    static ref PREAMBLE: Regex = Regex::new(r"Game (\d+):").unwrap();
    static ref PAIR: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

#[derive(Debug)]
struct ParsedData<'a> {
    gameNum: i32,
    games: Vec<HashMap<&'a str, i32>>,
}

fn parseInput(input: &str) -> ParsedData {
    let gameNum = PREAMBLE.captures(input).unwrap().get(1).unwrap().as_str();

    let games = input.split(": ")
        .last().unwrap()
        .split(";")
        .collect::<Vec<&str>>();

    let gameData = games.iter().map(|game | {
        let sets = game.split(",").collect::<Vec<&str>>();
        let sets: Vec<(&str, i32)> = sets.iter().map(|set| {
            let captures = &PAIR.captures(set).unwrap();

            let num = captures.get(1).unwrap()
                .as_str().to_string()
                .parse::<i32>().unwrap();
            let name = captures.get(2).unwrap().as_str();

            (name, num)
        }).collect::<Vec<(&str, i32)>>();

        sets.into_iter().collect::<HashMap<&str, i32>>()
    }).collect::<Vec<HashMap<&str, i32>>>();

    ParsedData {
        gameNum: gameNum.to_string().parse::<i32>().unwrap(),
        games: gameData,
    }
}

fn get_power(data: ParsedData) -> i32 {
    let mut max_map: HashMap<&str, i32> = HashMap::new();
    let colors = ["red", "green", "blue"];

    data.games.iter().for_each(|set| {
        for color in colors.iter() {
            let sample = *set.get(color).unwrap_or(&0);
            let current_max = *max_map.get(color).unwrap_or(&0);

            if (sample > current_max) {
                max_map.insert(color, sample);
            }
        }
    });

    let counts = colors.map(|c| *max_map.get(c).unwrap_or(&1));
    counts.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[no_mangle]
    fn example() {
        let cases = [
            get_power(parseInput(r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")),
            get_power(parseInput(r"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")),
            get_power(parseInput(r"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")),
            get_power(parseInput(r"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")),
            get_power(parseInput(r"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"))
        ];

        itertools::assert_equal(cases.iter(), [48, 12, 1560, 630, 36].iter());

        assert_eq!(cases.iter().sum::<i32>(), 2286);
    }
}