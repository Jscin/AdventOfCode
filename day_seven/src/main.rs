use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    HighCard, // Weakest
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind, // Strongest
}

#[derive(Debug)]
struct Card {
    cards: Vec<char>,
    bet: u32,
}

impl Card {
    fn new(cards: Vec<char>, bet: u32) -> Card {
        Card { cards, bet }
    }

    fn get_hand(&self) -> Hand {
        let mut counts = HashMap::new();

        // Count occurrences of each card
        for &card in &self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut pairs = 0;
        let mut three_of_a_kind = false;

        for &count in counts.values() {
            match count {
                2 => pairs += 1,
                3 => three_of_a_kind = true,
                4 => return Hand::FourOfAKind,
                5 => return Hand::FiveOfAKind,
                _ => (),
            }
        }

        if three_of_a_kind && pairs > 0 {
            return Hand::FullHouse;
        } else if three_of_a_kind {
            return Hand::ThreeOfAKind;
        } else if pairs == 2 {
            return Hand::TwoPairs;
        } else if pairs == 1 {
            return Hand::OnePair;
        }

        Hand::HighCard
    }
}

fn main() {
    let contents = fs::read_to_string("ex.txt").expect("Something went wrong reading the file");

    let cards = contents
        .split_terminator('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let cards = cards
        .iter()
        .map(|x| {
            let cards = x[0].chars().collect::<Vec<char>>();
            let bet = x[1].parse::<u32>().unwrap();
            Card::new(cards, bet)
        })
        .collect::<Vec<Card>>();

    let hands = &cards.iter().map(|x| x.get_hand()).collect::<Vec<Hand>>();
    for (hand, cards) in hands.iter().zip(cards.iter()) {
        let mut hand_count = 0;
        let mut rank = 0;
        for hand2 in hands.iter() {
            if hand == hand2 {
                hand_count += 1;
            }
        }
        if hand_count == 1 {
            rank = 1;
        } else if hand_count > 1 {
        }
    }
}
