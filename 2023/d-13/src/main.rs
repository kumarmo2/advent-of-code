#![allow(
    dead_code,
    unused_mut,
    unused_variables,
    unused_must_use,
    unused_assignments,
    unreachable_code
)]
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("{}", part1(input));
}

#[derive(Debug)]
enum Split {
    V(u32),
    H(u32),
}

fn part1(input: &str) -> u32 {
    let mut arr: [[char; 100]; 100] = [['.'; 100]; 100];

    let (h_split, v_split) = input
        .split("\n\n")
        .map(|matrix| {
            let rows = matrix.lines().count();
            let cols = matrix.lines().next().unwrap().chars().count();

            matrix.lines().enumerate().for_each(|(row, line)| {
                line.chars().enumerate().for_each(|(col, ch)| {
                    arr[row][col] = ch;
                })
            });
            return do_matrix(&arr[0..rows], rows as i32, cols as i32);
        })
        .fold((0, 0), |(h_split, v_split), split| match split {
            Split::V(v) => (h_split, v_split + v),
            Split::H(h) => (h + h_split, v_split),
        });
    return h_split * 100 + v_split;
}

fn do_matrix<const N: usize>(backing_matrix: &[[char; N]], rows: i32, cols: i32) -> Split {
    for k in 1..cols {
        let mut j = k;
        let mut i = k - 1;
        while i > -1 && j < cols {
            let mut items_are_mirrored = true;
            for row in 0..rows {
                if backing_matrix[row as usize][i as usize]
                    != backing_matrix[row as usize][j as usize]
                {
                    items_are_mirrored = false;
                    break;
                }
            }
            if !items_are_mirrored {
                break;
            }
            i -= 1;
            j += 1;
        }
        if i < 0 || j >= cols {
            return Split::V(k as u32);
        }
    }

    for k in 1..rows {
        let mut j = k;
        let mut i = k - 1;
        while i > -1 && j < rows {
            let mut items_are_mirrored = true;
            for col in 0..cols {
                if backing_matrix[i as usize][col as usize]
                    != backing_matrix[j as usize][col as usize]
                {
                    items_are_mirrored = false;
                    break;
                }
            }
            if !items_are_mirrored {
                break;
            }
            i -= 1;
            j += 1;
        }
        if i < 0 || j >= rows {
            return Split::H(k as u32);
        }
    }
    unreachable!()
}
