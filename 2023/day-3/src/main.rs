#![allow(dead_code, unused_mut, unused_variables, unused_must_use)]

fn main() {
    println!("Hello, world!");
    println!("{}: ", part2(include_str!("../input-prod.txt")));
}

fn part2(input: &str) -> u64 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut aux: Vec<Vec<(u64, u64, u64)>> = Vec::new(); // the third item in the tuple signifies
                                                         // the total number of adjacent numbers for the symbol
    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        aux.push(vec![(0, 0, 0); cols]);
    }

    let mut i: usize = 0;
    loop {
        let mut j: usize = 0;
        loop {
            if j >= cols {
                break;
            }
            let ch = matrix[i][j];
            if !ch.is_digit(10) {
                j += 1;
                continue;
            }
            let (value, length) = get_int_val(&matrix[i][j..]);
            let mut found_symbol = false;
            if check_for_symbol_in_left(j, &matrix[i]) && matrix[i][j - 1] == '*' {
                let mut tuple = aux[i][j - 1];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i][j - 1] = tuple;
            }
            if check_for_symbol_in_right(j, length, cols, &matrix[i])
                && matrix[i][j + length] == '*'
            {
                let mut tuple = aux[i][j + length];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i][j + length] = tuple;
            }

            if check_for_symbol_in_top_left_diagonally(i, j, rows, cols, &matrix)
                && matrix[i - 1][j - 1] == '*'
            {
                let mut tuple = aux[i - 1][j - 1];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i - 1][j - 1] = tuple;
                // println!("...found_symbol on top  left");
                // total = total + value;
            }
            if check_for_symbol_in_top_right_diagonally(i, j, length, rows, cols, &matrix)
                && matrix[i - 1][j + length] == '*'
            {
                let mut tuple = aux[i - 1][j + length];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i - 1][j + length] = tuple;
            }
            if check_for_symbol_in_bottom_right_diagonally(i, j, length, rows, cols, &matrix)
                && matrix[i + 1][j + length] == '*'
            {
                println!(">> found in diagonal");
                let mut tuple = aux[i + 1][j + length];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i + 1][j + length] = tuple;
            }
            if check_for_symbol_in_bottom_left_diagonally(i, j, rows, cols, &matrix)
                && matrix[i + 1][j - 1] == '*'
            {
                let mut tuple = aux[i + 1][j - 1];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i + 1][j - 1] = tuple;
            }
            let result = check_for_symbol_in_above_row_in_parallel(i, j, length, &matrix);
            if result.0 && matrix[i - 1][result.1] == '*' {
                let mut tuple = aux[i - 1][result.1];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i - 1][result.1] = tuple;
            }
            let result = check_for_symbol_in_below_row_in_parallel(i, j, rows, length, &matrix);
            if result.0 && matrix[i + 1][result.1] == '*' {
                let mut tuple = aux[i + 1][result.1];
                tuple.2 = tuple.2 + 1;
                if tuple.0 == 0 {
                    tuple.0 = value
                } else if tuple.1 == 0 {
                    tuple.1 = value;
                }
                aux[i + 1][result.1] = tuple;
            }
            j = j + length;
        }
        i += 1;
        if i >= rows {
            break;
        }
    }

    let total: u64 = aux
        .into_iter()
        .flatten()
        .filter_map(|tuple| {
            if tuple.2 != 2 {
                return None;
            }
            Some(tuple.0 * tuple.1)
        })
        .sum();

    // .collect::<Vec<(u64, u64, u64)>>();
    total
}

fn part1(input: &str) -> u64 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // println!("{:?}", matrix);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut total = 0;

    let mut i: usize = 0;
    loop {
        let mut j: usize = 0;
        loop {
            if j >= cols {
                break;
            }
            let ch = matrix[i][j];
            if !ch.is_digit(10) {
                j += 1;
                continue;
            }
            let (value, length) = get_int_val(&matrix[i][j..]);
            let mut found_symbol = false;
            // found_symbol = check_for_symbol_in_left(j, &matrix[i]);
            // println!("found_symbol on left: {}", found_symbol);
            // if found_symbol {
            //     total = total + value;
            // }
            //
            if check_for_symbol_in_left(j, &matrix[i]) {
                println!("...found_symbol on left");
                total = total + value;
            } else if check_for_symbol_in_right(j, length, cols, &matrix[i]) {
                println!("...found_symbol on right");
                total = total + value;
            } else if check_for_symbol_in_top_left_diagonally(i, j, rows, cols, &matrix) {
                println!("...found_symbol on top  left");
                total = total + value;
            } else if check_for_symbol_in_top_right_diagonally(i, j, length, rows, cols, &matrix) {
                println!("...found_symbol on top  right");
                total = total + value;
            } else if check_for_symbol_in_bottom_right_diagonally(i, j, length, rows, cols, &matrix)
            {
                println!("...found_symbol on bottom  right");
                total = total + value;
            } else if check_for_symbol_in_bottom_left_diagonally(i, j, rows, cols, &matrix) {
                println!("...found_symbol on bottom  left");
                total = total + value;
            } else if check_for_symbol_in_above_row_in_parallel(i, j, length, &matrix).0 {
                println!("...found_symbol on above row in parallel");
                total = total + value;
            } else if check_for_symbol_in_below_row_in_parallel(i, j, rows, length, &matrix).0 {
                println!("...found_symbol on below row in parallel");
                total = total + value;
            }

            // found_symbol = check_for_symbol_in_right(j, length, cols, &matrix[i]);
            // println!("found_symbol on right: {}", found_symbol);
            // if found_symbol {
            //     total = total + value;
            // }

            /*
             *  check for symbol at
             *  1. check at j  - 1.
             *  2. j + length
             *  3. diagonal, (i - 1, j - 1), (i - 1, j + 1), (i + 1, j + 1, i + 1, j - 1).
             *  4. from j to j + length - 1, in i - 1  and i + 1 row.
             * */

            j = j + length;
        }
        i += 1;
        if i >= rows {
            break;
        }
    }
    total
}

fn check_for_symbol_in_above_row_in_parallel(
    i: usize,
    j: usize,
    length: usize,
    matrix: &Vec<Vec<char>>,
) -> (bool, usize) {
    if i == 0 {
        (false, 0)
    } else {
        check_for_symbol_in_row_in_parallel(i - 1, j, length, matrix)
    }
}
fn check_for_symbol_in_below_row_in_parallel(
    i: usize,
    j: usize,
    rows: usize,
    length: usize,
    matrix: &Vec<Vec<char>>,
) -> (bool, usize) {
    if i == rows - 1 {
        (false, 0)
    } else {
        check_for_symbol_in_row_in_parallel(i + 1, j, length, matrix)
    }
}
fn check_for_symbol_in_row_in_parallel(
    row: usize,
    j: usize,
    length: usize,
    matrix: &Vec<Vec<char>>,
) -> (bool, usize) {
    let mut k = j;
    while k < j + length {
        if is_symbol(matrix[row][k]) {
            return (true, k);
        }
        k += 1;
    }
    (false, 0)
}

fn check_for_symbol_in_top_left_diagonally(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    matrix: &Vec<Vec<char>>,
) -> bool {
    if i == 0 || j == 0 {
        false
    } else {
        is_symbol(matrix[i - 1][j - 1])
    }
}
fn check_for_symbol_in_bottom_left_diagonally(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    matrix: &Vec<Vec<char>>,
) -> bool {
    if i == rows - 1 || j == 0 {
        false
    } else {
        is_symbol(matrix[i + 1][j - 1])
    }
}
fn check_for_symbol_in_top_right_diagonally(
    i: usize,
    j: usize,
    length: usize,
    rows: usize,
    cols: usize,
    matrix: &Vec<Vec<char>>,
) -> bool {
    if i == 0 || j + length >= cols - 1 {
        false
    } else {
        is_symbol(matrix[i - 1][j + length])
    }
}
fn check_for_symbol_in_bottom_right_diagonally(
    i: usize,
    j: usize,
    length: usize,
    rows: usize,
    cols: usize,
    matrix: &Vec<Vec<char>>,
) -> bool {
    if i == rows - 1 || j + length >= cols - 1 {
        false
    } else {
        is_symbol(matrix[i + 1][j + length])
    }
}

fn check_for_symbol_in_left(j: usize, row: &Vec<char>) -> bool {
    if j == 0 {
        return false;
    }
    is_symbol(row[j - 1])
}
fn check_for_symbol_in_right(j: usize, length: usize, cols_count: usize, row: &Vec<char>) -> bool {
    if j + length >= cols_count - 1 {
        return false;
    }
    // println!("right symbol: {}", row[j + length]);
    is_symbol(row[j + length])
}

fn is_symbol(ch: char) -> bool {
    !ch.is_digit(10) && !ch.is_alphabetic() && ch != '.'
}

/// returns the numeric value and the length of the number.
fn get_int_val(slice: &[char]) -> (u64, usize) {
    let length = slice.iter().take_while(|ch| ch.is_digit(10)).count();
    let mut multiplier = (10 as u32).pow(length as u32 - 1);
    let mut value: u32 = 0;
    for ch in slice[0..length].iter() {
        value = value + ch.to_digit(10).unwrap() * multiplier;
        multiplier = multiplier / 10;
    }
    println!(">>> returning, value: {}, length: {}", value, length);
    (value as u64, length)
}
