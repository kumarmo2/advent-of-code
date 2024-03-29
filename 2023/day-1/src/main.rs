#![allow(unused_variables, unused_mut, dead_code)]
mod kmp;
mod tests;

const MATRIX: [[i64; 100]; 100] = [[0; 100]; 100];
use std::{collections::HashMap, env};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    // println!("args: {:?}", args);
    //
    // // let occurence = kmp::find_occurences("aba", "ba");
    // let mut lps: [usize; 1000000] = [0; 1000000];
    // let occurence = kmp::find_occurences(args[1].as_str(), args[2].as_str(), &mut lps[..]);
    // println!("{:?}", occurence);
    println!("day2: {}", day2());
}

pub fn day2() -> u64 {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits_map = HashMap::new();
    digits_map.insert("one", 1);
    digits_map.insert("two", 2);
    digits_map.insert("three", 3);
    digits_map.insert("four", 4);
    digits_map.insert("five", 5);
    digits_map.insert("six", 6);
    digits_map.insert("seven", 7);
    digits_map.insert("eight", 8);
    digits_map.insert("nine", 9);
    digits_map.insert("1", 1);
    digits_map.insert("2", 2);
    digits_map.insert("3", 3);
    digits_map.insert("4", 4);
    digits_map.insert("5", 5);
    digits_map.insert("6", 6);
    digits_map.insert("7", 7);
    digits_map.insert("8", 8);
    digits_map.insert("9", 9);

    let input = include_str!("../input-1.txt");
    let lines: Vec<_> = input.split('\n').collect();
    let mut sum: u64 = 0;

    let mut lps: [usize; 1000] = [0; 1000];
    for line in lines.iter() {
        let val = inner_loop(line, &digits_map, &mut lps[..]);
        sum += val;
    }
    sum
}

pub(crate) fn inner_loop(line: &str, digits_map: &HashMap<&str, u32>, lps: &mut [usize]) -> u64 {
    let mut min_index = usize::max_value();
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut max_index = usize::min_value();

    for key in digits_map.keys() {
        let Some(occurences) = kmp::find_occurences(&line, &key, &mut lps[..]) else {
            continue;
        };
        let n = occurences.indexes().len();
        let local_min_index = occurences.indexes()[0];
        let local_max_index = occurences.indexes()[n - 1];
        if local_min_index < min_index {
            min_index = local_min_index;
            first_digit = *digits_map.get(key).unwrap();
        }
        if local_max_index >= max_index {
            max_index = local_max_index;
            last_digit = *digits_map.get(key).unwrap();
        }
    }
    return first_digit as u64 * 10 + last_digit as u64;
}

pub fn contains_at(input: &str, pattern: &str) -> Option<usize> {
    todo!()
}

pub fn day1() -> u64 {
    let input = include_str!("../input-1.txt");
    let lines: Vec<_> = input.split('\n').collect();
    let mut sum: u64 = 0;
    println!("lines: {}", lines.len()); // lines.len()

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

        // println!("first_digit: {}, last_digit: {}", first_digit, last_digit);

        sum = sum + first_digit as u64 * 10 + last_digit as u64;
    }
    println!("sum: {}", sum);
    return sum;
}
