#![allow(
    unused_mut,
    unused_variables,
    dead_code,
    unused_must_use,
    unused_assignments
)]
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("part1: {}", part1(input));
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut galaxies: Vec<(usize, usize)> = vec![];

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let item = input[i][j];
            if item == '#' {
                galaxies.iter().for_each(|galaxy| {
                    // get the dist between galaxy and i,j
                    sum = sum
                        + (i as i32 - galaxy.0 as i32).abs()
                        + (j as i32 - galaxy.1 as i32).abs();
                });
                galaxies.push((i, j));
            }
        }
    }

    galaxies.sort_by(|g1, g2| g1.1.cmp(&g2.1));
    // println!("galaxies: {:?}", galaxies);
    // println!("sum: {}", sum);

    for j in 0..input[0].len() {
        let mut found_no_galaxy = true;
        for i in 0..input.len() {
            if input[i][j] == '#' {
                found_no_galaxy = false;
                break;
            }
        }
        if found_no_galaxy {
            let left_galaxies = galaxies.iter().take_while(|galaxy| galaxy.1 < j).count();
            let right_galaxies = galaxies.len() - left_galaxies;
            let to_add = left_galaxies * right_galaxies;
            sum = sum + to_add as i32;
            // println!(
            //     "at j: {}, left_galaxies: {}, right_galaxies: {}, sum: {}",
            //     j, left_galaxies, right_galaxies, sum
            // );
        }
    }
    galaxies.sort_by(|g1, g2| g1.0.cmp(&g2.0));
    // println!(">>> sorted horizontall: {:?}", galaxies);

    for i in 0..input.len() {
        let mut found_no_galaxy = true;
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                found_no_galaxy = false;
                break;
            }
        }
        if found_no_galaxy {
            let upper_galaxies = galaxies.iter().take_while(|galaxy| galaxy.0 < i).count();
            let lower_galaxies = galaxies.len() - upper_galaxies;
            let to_add = upper_galaxies * lower_galaxies;
            sum = sum + to_add as i32;
            // println!(
            //     "at i: {}, upper_galaxies: {}, lower_galaxies: {}, sum: {}",
            //     i, upper_galaxies, lower_galaxies, sum
            // );
        }
    }
    sum
}
