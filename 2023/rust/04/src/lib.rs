use regex::Regex;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub numbers: Vec<usize>,
    pub winning: Vec<usize>,
}

impl Card {
    pub fn new(line: &str) -> Option<Card> {
        let re = Regex::new(r"Card\s*(\d+)\s*:(.*)\|(.*)").unwrap();
        if let Some(captures) = re.captures(line) {
            let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let winning = captures.get(2).unwrap()
                .as_str().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let numbers = captures.get(3).unwrap()
                .as_str().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            let card = Card {
                id: id,
                winning: winning,
                numbers: numbers
            };

            card.find_winners();

            return Some(card)
        }

        None
    }

    pub fn find_winners(self: &Self) -> Vec<usize> {
        let mut winners = Vec::<usize>::new();

        for number in &self.numbers {
            if self.winning.contains(number) {
                winners.push(*number);
            }
        }

        winners
    }


    pub fn calculate_score(self: &Self) -> usize {
        let matches = self.find_winners();

        if matches.len() == 0 {
            return 0;
        }

        1 << matches.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::new(input).unwrap();
        print!("{:?}", card);

        assert_eq!(card.id, 1);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);

        let winners = card.find_winners();
        assert_eq!(vec![83, 86, 17, 48], winners);

        let score = card.calculate_score();
        assert_eq!(score, 8);
    }

    #[test]
    fn parse_2() {
        let input = r"Card 1: 1 2 3 4    05 | 83 86  6 31 17  9 48 53";
        let card = Card::new(input).unwrap();
        print!("{:?}", card);

        assert_eq!(card.id, 1);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.winning, vec![1, 2, 3, 4, 5]);

        let winners = card.find_winners();
        assert_eq!(Vec::<usize>::new(), winners);

        let score = card.calculate_score();
        assert_eq!(score, 0);
    }

    #[test]
    fn parse_3() {
        let input = r"Card 1: 1 2 9 4    05 | 83 86  6 31 17  9 48 53";
        let card = Card::new(input).unwrap();
        print!("{:?}", card);

        assert_eq!(card.id, 1);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.winning, vec![1, 2, 9, 4, 5]);

        let winners = card.find_winners();
        assert_eq!(vec![9], winners);

        let score = card.calculate_score();
        assert_eq!(score, 1);
    }
}
