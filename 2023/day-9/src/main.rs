#![allow(unused_mut, unused_variables, dead_code, unreachable_code)]
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("{}, ", day1(input));
}

fn day1(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum: i32 = 0;
    // for j in 0..lines.len() {
    for line in lines.into_iter() {
        let mut last_numbers: Vec<i32> = Vec::new();
        let mut diffs: Vec<Vec<i32>> = Vec::new();
        // println!("line: {:?}", line);
        let mut temp_line = line;

        loop {
            if temp_line.iter().all(|val| *val == 0) {
                break;
            }
            let n = temp_line.len();
            let mut next_line = Vec::new();
            for i in 1..=n - 1 {
                next_line.push(temp_line[i] - temp_line[i - 1]);
                if i == n - 1 {
                    last_numbers.push(temp_line[i]);
                }
            }
            temp_line = next_line;
        }
        // println!("last_numbers{:?}", last_numbers);
        let mut prev_number = 0; // this is the last Zero.
        let predicted_number = last_numbers
            .iter()
            .rev()
            .fold(0, |prev_number, number| number + prev_number);
        // println!("predicted_number: {}", predicted_number);
        sum += predicted_number as i32;
    }
    sum
}
