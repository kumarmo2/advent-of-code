use std::fs;

fn main() {
    println!("Hello, world!");
    let content =
        fs::read_to_string("/home/manya/code/advent-of-code/2022/day-1/input.txt").unwrap();

    println!("{}", part_2(&content));
}

/// Find max Calories part_1.
fn part_1(content: &str) -> u32 {
    let mut max_sum = 0;
    let mut sum: u32 = 0;
    let lines = content.split("\n\n");

    for group_of_calories in lines {
        sum = 0;
        for calorie in group_of_calories.split_whitespace() {
            let num = u32::from_str_radix(calorie, 10).unwrap();
            sum += num;
        }
        if sum > max_sum {
            max_sum = sum;
        }
    }
    return max_sum;
}

fn part_2(content: &str) -> u32 {
    let mut first_highest_sum = 0;
    let mut second_highest_sum = 0;
    let mut third_highest_sum = 0;
    let mut sum: u32 = 0;
    let lines = content.split("\n\n");

    for group_of_calories in lines {
        sum = 0;
        for calorie in group_of_calories.split_whitespace() {
            let num = u32::from_str_radix(calorie, 10).unwrap();
            sum += num;
        }
        if sum > first_highest_sum {
            third_highest_sum = second_highest_sum;
            second_highest_sum = first_highest_sum;
            first_highest_sum = sum;
        } else if sum > second_highest_sum {
            third_highest_sum = second_highest_sum;
            second_highest_sum = sum;
        } else if sum > third_highest_sum {
            third_highest_sum = sum;
        }
    }
    return first_highest_sum + second_highest_sum + third_highest_sum;
}

#[test]
fn it_works_for_just_one_num() {
    let content = "23";
    assert_eq!(23, part_1(content));
}

#[test]
fn it_works() {
    let content = "10\n20\n\n50";
    assert_eq!(50, part_1(content));

    let content = "50\n20\n\n50";
    assert_eq!(70, part_1(content));

    let content = "100\n\n50";
    assert_eq!(100, part_1(content));
}
