#![allow(dead_code, unused_variables, unused_mut, unused_must_use)]
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("part2: {}", part2(input));
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
    is_deleted: bool,
}

struct Box<'a> {
    lenses: Vec<Lens<'a>>,
}

fn part2(input: &str) -> u32 {
    let mut boxes = Vec::with_capacity(256);
    for i in 0..256 {
        boxes.push(Box { lenses: Vec::new() });
    }

    // input.split(",").map(|word| {
    for word in input.split(",") {
        let word = word.trim();
        let len = word.len();
        let has_dash = &word[len - 1..] == "-";
        if has_dash {
            let word = &word[0..len - 1];
            let box_num = get_hash(word);
            let mut bx = &mut boxes[box_num as usize];
            let lens = bx
                .lenses
                .iter_mut()
                .find(|lens| !lens.is_deleted && lens.label == word);
            if let Some(lens) = lens {
                lens.is_deleted = true;
            }
        } else {
            let (index, _) = word.chars().enumerate().find(|(i, ch)| *ch == '=').unwrap();
            let label = &word[0..index];
            let focal_length = word[index + 1..].parse::<u32>().unwrap();
            let box_num = get_hash(label);
            let mut bx = &mut boxes[box_num as usize];
            match bx
                .lenses
                .iter_mut()
                .find(|lens| !lens.is_deleted && lens.label == label)
            {
                Some(lens) => lens.focal_length = focal_length,
                None => bx.lenses.push(Lens {
                    label,
                    focal_length,
                    is_deleted: false,
                }),
            }
        }
    }
    let mut box_sum: u32 = 0;

    for (box_index, bx) in boxes.iter().enumerate() {
        let mut lens_index: usize = 0;
        let mut lens_sum: u32 = 0;
        for lens in bx.lenses.iter() {
            if lens.is_deleted {
                continue;
            }
            lens_sum += (box_index as u32 + 1) * (lens_index as u32 + 1) * lens.focal_length;
            lens_index += 1;
        }
        box_sum += lens_sum;
    }
    box_sum
}

fn part1(input: &str) -> u32 {
    input.split(",").map(|word| get_hash(word)).sum()
}

fn get_hash(word: &str) -> u32 {
    word.trim()
        .chars()
        .map(|ch| ch as u32)
        .fold(0, |sum, asci| ((sum + asci) * 17) % 256)
}
