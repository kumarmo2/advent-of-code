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

fn rec(fields: &str, nums: &[u32]) -> u32 {
    // println!("fields: {fields}, nums: {nums:?}");
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
        // println!("found '#'. Iterating over it, fields: {fields}, nums: {nums:?}, total: {total}");

        while count < nums[0] {
            if local_fields.starts_with("#") || local_fields.starts_with("?") {
                count += 1;
                local_fields = &local_fields[1..];
            } else {
                break;
            }
        }

        if count < nums[0] {
            // println!(
            //     "count: {count} was less than nums[0]: {}...returning now",
            //     nums[0]
            // );
            return 0;
        }

        if local_fields.starts_with("#") {
            // count == nums[0] && local_fields starts with '#'
            return 0;
        }
        if local_fields.starts_with("?") {
            return rec(&local_fields[1..], &nums[1..]);
        }
        return rec(local_fields, &nums[1..]);
    }

    if fields.starts_with("?") {
        // println!("found '?', replacing with firstly with '.', fields: {fields}, nums: {nums:?}, total: {total}");
        let count1 = rec(&fields[1..], nums); // replaced '?' with  '.'
                                              // println!("after replacing with '.', count: {count1}");

        // println!("for previously found '?', replacing with secondly with '#', fields: {fields}, nums: {nums:?}, total: {total}");
        let mut count2 = 0;
        let mut local_fields = fields;
        while count2 < nums[0] {
            if local_fields.starts_with("?") || local_fields.starts_with("#") {
                // replacing '?'
                // with '#' if encountered.
                count2 += 1;
                local_fields = &local_fields[1..];
            } else {
                break;
            }
        }
        if count2 < nums[0] {
            // println!("count2: {count2} was less than nums[0]: {}", nums[0]);
            return count1;
        }
        if local_fields.starts_with("#") {
            // count == nums[0] && local_fields starts with '#'
            // println!("count was equal to nums[0], but next char was '#', ...returning now");
            return count1;
        }
        if local_fields.starts_with("?") {
            return rec(&local_fields[1..], &nums[1..]) + count1;
        }
        // println!("count was equal to nums[0], and next char was not '?', local_fields: {local_fields:?}, ...returning now");
        return rec(local_fields, &nums[1..]) + count1;
    }
    unreachable!()
}
