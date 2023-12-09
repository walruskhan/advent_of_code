use std::collections::VecDeque;
use day04::*;

fn main() {
    let input = include_str!("../input.txt");

    let cards = input.lines()
        .map(|x| Card::new(x).unwrap())
        .collect::<Vec<Card>>();

    let mut cardset = VecDeque::<&Card>::new();
    let mut totalcards = Vec::<&Card>::new();

    for card in cards.iter() {
        cardset.push_back(card);
        totalcards.push(card);
    }

    let mut count = cardset.len();
    ;
    while cardset.len() > 0 {
        if let Some(card) = cardset.pop_front() {
            let matches = card.find_winners();

            let new_cards = cards.iter()
                .skip(card.id)
                .take(matches.len())
                .collect::<Vec<&Card>>();

            // println!("Won: {:?}", new_cards.iter().map(|x| x.id).collect::<Vec<usize>>());

            count += new_cards.len();
            new_cards.iter().for_each(|x| cardset.push_back(x));
            new_cards.iter().for_each(|x|totalcards.push(x));
        }
    }

    // let mut sorted_total_cards = totalcards.iter()
    //     .map(|x| x.id)
    //     .collect::<Vec<usize>>();
    // sorted_total_cards.sort();
    println!("Count: {}", count);
    // println!("cards: {:?}", sorted_total_cards);
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::*;
    
    #[test]
    fn first() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        
        let cards = input.lines()
            .map(|x| Card::new(x).unwrap())
            .collect::<Vec<Card>>();
        
        let mut cardset = VecDeque::<&Card>::new();
        let mut totalcards = Vec::<&Card>::new();
        
        for card in cards.iter() {
            cardset.push_back(card);
            totalcards.push(card);
        }
        
        let mut count = cardset.len();
        ;
        while cardset.len() > 0 {
            if let Some(card) = cardset.pop_front() {
                let matches = card.find_winners();
                
                let new_cards = cards.iter()
                    .skip(card.id)
                    .take(matches.len())
                    .collect::<Vec<&Card>>();
                
                println!("Won: {:?}", new_cards.iter().map(|x| x.id).collect::<Vec<usize>>());
                
                count += new_cards.len();
                new_cards.iter().for_each(|x| cardset.push_back(x));
                new_cards.iter().for_each(|x|totalcards.push(x));
            }
        }
        
        let mut sorted_total_cards = totalcards.iter()
            .map(|x| x.id)
            .collect::<Vec<usize>>();
        sorted_total_cards.sort();
        println!("Count: {}", count);
        println!("cards: {:?}", sorted_total_cards);
    }
}