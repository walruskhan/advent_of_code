use std::collections::HashMap;
use lazy_static::lazy_static;
use day02::add;
use regex::{Captures, Regex};

fn main() {
    let input = include_str!("../input.txt");
    println!("Total: {}", get_total(input));
}

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

fn get_total(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parseInput(line))
        .filter(|data| {
            println!("{:?}", &data);

            let valid = data.games.iter().filter(|set| {
                set.get("red").unwrap_or(&0) <= &12 &&
                    set.get("green").unwrap_or(&0) <= &13 &&
                    set.get("blue").unwrap_or(&0) <= &14

            });
            valid.count() == data.games.len()
        })
        .fold(0, |acc, data| {
            acc + data.gameNum
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[no_mangle]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let total = get_total(input);
        assert_eq!(total, 8);
    }
}