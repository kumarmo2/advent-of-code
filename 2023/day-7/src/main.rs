#![allow(dead_code, unused_variables, unused_mut, unused_must_use)]

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Hash)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

fn get_card_from_char(ch: char) -> Card {
    match ch {
        'A' => Card::Ace,
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Hand<'a> {
    FiveOfKind(&'a str),
    FourOfKind(&'a str),
    FullHouse(&'a str),
    ThreeOfKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FiveOfKind(l0), Self::FiveOfKind(r0)) => l0 == r0,
            (Self::FourOfKind(l0), Self::FourOfKind(r0)) => l0 == r0,
            (Self::FullHouse(l0), Self::FullHouse(r0)) => l0 == r0,
            (Self::ThreeOfKind(l0), Self::ThreeOfKind(r0)) => l0 == r0,
            (Self::TwoPair(l0), Self::TwoPair(r0)) => l0 == r0,
            (Self::OnePair(l0), Self::OnePair(r0)) => l0 == r0,
            (Self::HighCard(l0), Self::HighCard(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl<'a> Eq for Hand<'a> {}

fn compare_hand_str(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars();
    let mut b_iter = b.chars();
    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some(a0), Some(b0)) if a0 == b0 => continue,
            (Some(a0), Some(b0)) => {
                let a_priority = get_priority(a0);
                let b_priority = get_priority(b0);
                let order = a_priority.cmp(&b_priority);
                return order;
            }
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::FiveOfKind(l0), Self::FiveOfKind(r0)) => Some(compare_hand_str(l0, r0)),
            (Self::FiveOfKind(l0), _) => Some(std::cmp::Ordering::Greater),

            (Self::FourOfKind(l0), Self::FiveOfKind(_)) => Some(Ordering::Less),
            (Self::FourOfKind(l0), Self::FourOfKind(l1)) => Some(compare_hand_str(l0, l1)),
            (Self::FourOfKind(l0), _) => Some(std::cmp::Ordering::Greater),

            (Self::FullHouse(l0), Self::FiveOfKind(_) | Self::FourOfKind(_)) => {
                Some(Ordering::Less)
            }
            (Self::FullHouse(l0), Self::FullHouse(l1)) => Some(compare_hand_str(l0, l1)),
            (Self::FullHouse(l0), _) => Some(std::cmp::Ordering::Greater),

            (
                Self::ThreeOfKind(l0),
                Self::FiveOfKind(_) | Self::FourOfKind(_) | Self::FullHouse(_),
            ) => Some(Ordering::Less),
            (Self::ThreeOfKind(l0), Self::ThreeOfKind(r0)) => Some(compare_hand_str(l0, r0)),
            (Self::ThreeOfKind(l0), _) => Some(Ordering::Greater),

            (
                Self::TwoPair(l0),
                Self::FiveOfKind(_)
                | Self::FourOfKind(_)
                | Self::FullHouse(_)
                | Self::ThreeOfKind(_),
            ) => Some(Ordering::Less),
            (Self::TwoPair(l0), Self::TwoPair(r0)) => Some(compare_hand_str(l0, r0)),
            (Self::TwoPair(l0), _) => Some(Ordering::Greater),
            (
                Self::OnePair(l0),
                Self::FiveOfKind(_)
                | Self::FourOfKind(_)
                | Self::FullHouse(_)
                | Self::ThreeOfKind(_)
                | Self::TwoPair(_),
            ) => Some(Ordering::Less),
            (Self::OnePair(l0), Self::OnePair(r0)) => Some(compare_hand_str(l0, r0)),
            (Self::OnePair(l0), _) => Some(Ordering::Greater),
            (
                Self::HighCard(l0),
                Self::FiveOfKind(_)
                | Self::FourOfKind(_)
                | Self::FullHouse(_)
                | Self::ThreeOfKind(_)
                | Self::TwoPair(_)
                | Self::OnePair(_),
            ) => Some(Ordering::Less),
            (Self::HighCard(l0), Self::HighCard(r0)) => Some(compare_hand_str(l0, r0)),
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_card_counts(hand_str: &str) -> HashMap<Card, i32> {
    let mut map: HashMap<Card, i32> = HashMap::new();
    for ch in hand_str.chars() {
        let card = get_card_from_char(ch);
        let count = match map.get(&card) {
            Some(count) => count + 1,
            None => 1,
        };
        map.insert(card, count);
    }
    map
}

fn get_priority(ch: char) -> i32 {
    match ch {
        'A' => 14,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        _ => unreachable!(),
    }
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(value: &'a str) -> Self {
        let card_to_count_map = get_card_counts(value);
        match card_to_count_map.len() {
            5 => Hand::HighCard(value),
            4 => Hand::OnePair(value),
            3 => {
                let has_three = card_to_count_map.iter().any(|(_, count)| *count == 3);
                if has_three {
                    Hand::ThreeOfKind(value)
                } else {
                    Hand::TwoPair(value)
                }
            }
            2 => match card_to_count_map.iter().any(|(_, count)| *count == 3) {
                true => Hand::FullHouse(value),
                false => Hand::FourOfKind(value),
            },
            1 => Hand::FiveOfKind(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Row<'a> {
    hand: Hand<'a>,
    bid: u32,
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("{}, ", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut rows = input
        .lines()
        .map(|line| {
            let mut items = line.split(' ');
            let hand_str = items.next().unwrap();
            let bid = items.next().unwrap().parse::<u32>().unwrap();
            let hand = Hand::from(hand_str);
            Row { hand, bid }
        })
        .collect::<Vec<Row>>();
    rows.sort_by(|a, b| {
        let order = a.hand.cmp(&b.hand);
        return order;
    });
    let mut sum: u32 = 0;
    rows.iter().enumerate().for_each(|(i, row)| {
        sum = sum + row.bid * (i + 1) as u32;
    });
    sum
}
