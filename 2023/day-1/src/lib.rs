#![allow(unused_variables)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn day1() -> u64 {
    let input = include_str!("../input-1.txt");
    let lines: Vec<_> = input.split('\n').collect();
    let mut sum: u64 = 0;
    for line in lines.iter() {
        let mut first_digit = line.chars().skip_while(|ch| !ch.is_digit(10));
        let mut last_digit = line.chars().rev().skip_while(|ch| !ch.is_digit(10));
        let first_digit = match first_digit.next() {
            Some(ch) => ch.to_digit(10).unwrap(),
            None => 0,
        };
        let last_digit = match last_digit.next() {
            Some(ch) => ch.to_digit(10).unwrap(),
            None => 0,
        };

        sum = sum + first_digit as u64 + last_digit as u64;
    }
    println!("sum: {}", sum);
    return sum;
}
