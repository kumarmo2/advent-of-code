#![allow(dead_code, unused_mut, unused_variables, unused_must_use)]
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("sum: {}", part1(input, 12, 13, 14));
}

enum Ball {
    Red(u64),
    Green(u64),
    Blue(u64),
}
struct Game {
    id: u64,
    balls: Vec<Ball>,
}

fn part1(input: &str, max_red: u64, max_green: u64, max_blue: u64) -> u64 {
    let sum: u64 = input
        .lines()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .filter_map(|items| {
            let game_id = items[0].trim()[5..].parse::<u64>().unwrap();
            let is_valid_game = items[1]
                .trim()
                .split(';')
                .map(|reveal| {
                    let balls = reveal
                        .trim()
                        .split(',')
                        .map(|balls_of_single_type| {
                            let mut items = balls_of_single_type.trim().split(' ');
                            let count = items.next().unwrap().parse::<u64>().unwrap();
                            let color = items.next().unwrap();
                            match color {
                                "red" => Ball::Red(count),
                                "blue" => Ball::Blue(count),
                                "green" => Ball::Green(count),
                                _ => unreachable!(),
                            }
                        })
                        .collect::<Vec<Ball>>();
                    balls
                })
                .all(|balls_in_single_reveal| {
                    for ball in balls_in_single_reveal {
                        let should_countinue = match ball {
                            Ball::Red(count) => count <= max_red,
                            Ball::Green(count) => count <= max_green,
                            Ball::Blue(count) => count <= max_blue,
                        };
                        if !should_countinue {
                            return false;
                        }
                    }
                    true
                });

            if is_valid_game {
                Some(game_id)
            } else {
                None
            }
        })
        .sum();
    sum
}
