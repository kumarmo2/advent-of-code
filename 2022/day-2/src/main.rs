use std::{collections::HashMap, fs, hash::Hash};

#[derive(Hash, PartialEq, Eq)]
enum Attack {
    Rock,
    Paper,
    Scissor,
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl Attack {
    fn get_total_point(&self, opponent: &Attack) -> i32 {
        match (self, opponent) {
            (Attack::Rock, Attack::Rock) => 4,
            (Attack::Rock, Attack::Paper) => 1,
            (Attack::Rock, Attack::Scissor) => 7,
            (Attack::Paper, Attack::Paper) => 5,
            (Attack::Paper, Attack::Rock) => 8,
            (Attack::Paper, Attack::Scissor) => 2,
            (Attack::Scissor, Attack::Scissor) => 6,
            (Attack::Scissor, Attack::Rock) => 3,
            (Attack::Scissor, Attack::Paper) => 9,
            // _ => unreachable!(),
        }
    }

    fn find_my_attack_if_opponent_attack_and_round_result_is_given(
        opponent: &Attack,
        round_result: &RoundResult,
    ) -> Attack {
        match (opponent, round_result) {
            (Attack::Rock, RoundResult::Win) => Attack::Paper,
            (Attack::Rock, RoundResult::Lose) => Attack::Scissor,
            (Attack::Rock, RoundResult::Draw) => Attack::Rock,

            (Attack::Paper, RoundResult::Win) => Attack::Scissor,
            (Attack::Paper, RoundResult::Lose) => Attack::Rock,
            (Attack::Paper, RoundResult::Draw) => Attack::Paper,

            (Attack::Scissor, RoundResult::Win) => Attack::Rock,
            (Attack::Scissor, RoundResult::Lose) => Attack::Paper,
            (Attack::Scissor, RoundResult::Draw) => Attack::Scissor,
        }
    }
}

fn main() {
    println!("Hello, world!");
    // let mut symbol_to_score = HashMap::new();
    let mut symbol_to_attack = HashMap::new();
    symbol_to_attack.insert("A", Attack::Rock);
    symbol_to_attack.insert("B", Attack::Paper);
    symbol_to_attack.insert("C", Attack::Scissor);

    symbol_to_attack.insert("X", Attack::Rock);
    symbol_to_attack.insert("Y", Attack::Paper);
    symbol_to_attack.insert("Z", Attack::Scissor);

    let mut symbol_to_result = HashMap::new();

    symbol_to_result.insert("X", RoundResult::Lose);
    symbol_to_result.insert("Y", RoundResult::Draw);
    symbol_to_result.insert("Z", RoundResult::Win);

    let content =
        fs::read_to_string("/home/manya/code/advent-of-code/2022/day-2/input.txt").unwrap();
    // println!("{}", part1(&content, &symbol_to_attack));
    println!("{}", part2(&content, &symbol_to_attack, &symbol_to_result));
}

fn part1(content: &str, symbol_to_attack: &HashMap<&str, Attack>) -> i32 {
    let mut points = 0;
    for line in content.lines() {
        let mut symbol_iter = line.split_whitespace();
        let Some(opponent_symbol ) = symbol_iter.next() else {
            break;

        };
        let opponent_symbol = symbol_to_attack.get(opponent_symbol).unwrap();
        let my_attack_symbol = symbol_to_attack.get(symbol_iter.next().unwrap()).unwrap();
        points += my_attack_symbol.get_total_point(opponent_symbol);
    }
    points
}

fn part2(
    content: &str,
    symbol_to_attack: &HashMap<&str, Attack>,
    symbol_to_result: &HashMap<&str, RoundResult>,
) -> i32 {
    let mut points = 0;
    for line in content.lines() {
        let mut symbol_iter = line.split_whitespace();
        let Some(opponent_symbol ) = symbol_iter.next() else {
            break;

        };
        let opponent_symbol = symbol_to_attack.get(opponent_symbol).unwrap();
        let attack_result = symbol_to_result.get(symbol_iter.next().unwrap()).unwrap();

        let my_attack = Attack::find_my_attack_if_opponent_attack_and_round_result_is_given(
            opponent_symbol,
            attack_result,
        );

        points += my_attack.get_total_point(opponent_symbol);
    }
    points
}
