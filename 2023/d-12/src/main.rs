#![allow(
    dead_code,
    unused_variables,
    unused_mut,
    unused_must_use,
    unused_assignments
)]

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("part1: {:?}", part1(input))
    // println!("part2: {:?}", part2(input))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");
            let fields = splits.next().unwrap();
            let nums = splits
                .next()
                .unwrap()
                .split(",")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let arrangements = rec(fields, nums.as_slice());
            arrangements
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");
            let fields = splits.next().unwrap();
            let nums = splits
                .next()
                .unwrap()
                .split(",")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut agg_fields = String::new();
            let mut agg_num: Vec<u32> = Vec::new();

            for i in 0..5 {
                agg_fields.push_str(fields);
                if i != 4 {
                    agg_fields.push('?');
                }
                for num in nums.iter() {
                    agg_num.push(*num)
                }
            }
            let arrangements = rec(&agg_fields, agg_num.as_slice());
            arrangements
        })
        .sum()
}

fn rec(fields: &str, nums: &[u32]) -> u32 {
    if nums.len() == 0 {
        if fields.contains("#") {
            return 0;
        }
        return 1;
    }

    if fields.len() == 0 && nums.len() > 0 {
        return 0;
    }

    if fields.starts_with(".") {
        return rec(&fields[1..], nums);
    }

    if fields.starts_with("#") {
        let mut count = 0;
        let mut local_fields = fields;

        while count < nums[0] {
            if local_fields.starts_with("#") || local_fields.starts_with("?") {
                count += 1;
                local_fields = &local_fields[1..];
            } else {
                break;
            }
        }

        if count < nums[0] {
            return 0;
        }

        if local_fields.starts_with("#") {
            return 0;
        }
        if local_fields.starts_with("?") {
            return rec(&local_fields[1..], &nums[1..]);
        }
        return rec(local_fields, &nums[1..]);
    }

    if fields.starts_with("?") {
        let count1 = rec(&fields[1..], nums); // replaced '?' with  '.'

        let mut count2 = 0;
        let mut local_fields = fields;
        while count2 < nums[0] {
            if local_fields.starts_with("?") || local_fields.starts_with("#") {
                count2 += 1;
                local_fields = &local_fields[1..];
            } else {
                break;
            }
        }
        if count2 < nums[0] {
            return count1;
        }
        if local_fields.starts_with("#") {
            return count1;
        }
        if local_fields.starts_with("?") {
            return rec(&local_fields[1..], &nums[1..]) + count1;
        }
        return rec(local_fields, &nums[1..]) + count1;
    }
    unreachable!()
}
