#![allow(dead_code, unused_variables, unused_mut, unused_must_use)]

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("part1: {}", part1(input)); // "part1(input);
}

fn part1(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '|' => Direction::NS,
                    '-' => Direction::EW,
                    'L' => Direction::NE,
                    'J' => Direction::NW,
                    '7' => Direction::SW,
                    'F' => Direction::SE,
                    '.' => Direction::None,
                    'S' => Direction::Origin,
                    _ => unreachable!("not handled input: {ch}"),
                })
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Vec<Direction>>>();

    let m = matrix.len();
    let n = matrix[0].len();

    let (start_row, start_col) = find_start_point(&matrix);
    let (next_row, next_col) = find_any_connecting_pipe((start_row, start_col), (m, n), &matrix);
    let mut visiting: Vec<Vec<bool>> = Vec::new();

    for i in 0..m {
        visiting.push(vec![false; n]);
    }
    let mut i = start_row;
    let mut j = start_col;
    visiting[i][j] = true;
    println!("startcode: {start_row},{start_col}");
    let cycle_length = visit((next_row, next_col), (m, n), &matrix, &mut visiting, 1);
    // println!("{cycle_length}");
    if cycle_length % 2 == 0 {
        return cycle_length / 2;
    }
    cycle_length / 2 + 1
}

fn visit(
    (i, j): (usize, usize),
    (m, n): (usize, usize),
    matrix: &Vec<Vec<Direction>>,
    visiting: &mut Vec<Vec<bool>>,
    steps_till_now: u32,
) -> u32 {
    visiting[i][j] = true;
    let curr_pipe = matrix[i][j];
    let Some([first_target, second_target]) = curr_pipe.target_coordinates((i as i32, j as i32))
    else {
        println!("returing from here");
        visiting[i][j] = false;
        return steps_till_now;
    };

    let (next_i, next_j) = first_target;
    if next_i >= 0 && next_i < m as i32 && next_j >= 0 && next_j < n as i32 {
        let next_i = next_i as usize;
        let next_j = next_j as usize;
        match (matrix[i][j], visiting[next_i][next_j]) {
            (_, false) => {
                let result = visit(
                    (next_i, next_j),
                    (m, n),
                    matrix,
                    visiting,
                    steps_till_now + 1,
                );
                visiting[i][j] = false;
                return result;
            }
            _ => (),
        };
    }

    let (next_i, next_j) = second_target;
    if next_i >= 0 && next_i < m as i32 && next_j >= 0 && next_j < n as i32 {
        let next_i = next_i as usize;
        let next_j = next_j as usize;
        match (matrix[i][j], visiting[next_i][next_j]) {
            (_, false) => {
                let result = visit(
                    (next_i, next_j),
                    (m, n),
                    matrix,
                    visiting,
                    steps_till_now + 1,
                );
                visiting[i][j] = false;
                return result;
            }
            _ => (),
        };
    }

    visiting[i][j] = false;
    return steps_till_now + 1;
}

#[derive(Clone, Debug, Copy)]
enum Direction {
    NS,     // |
    EW,     // -
    NE,     // L
    NW,     // J
    SW,     // 7
    SE,     // F
    None,   //.
    Origin, // S
}

impl Direction {
    fn can_move_left(dest_pipe: Direction) -> bool {
        match dest_pipe {
            Direction::Origin | Direction::EW | Direction::NW => true,
            _ => false,
        }
    }
    fn can_accept_from_below(dest_pipe: Direction) -> bool {
        match dest_pipe {
            Direction::Origin | Direction::NS | Direction::SW | Direction::SE => true,
            _ => false,
        }
    }
    fn can_accept_from_right(dest_pipe: Direction) -> bool {
        match dest_pipe {
            Direction::Origin | Direction::EW | Direction::SE | Direction::NE => true,
            _ => false,
        }
    }

    fn can_accept_from_above(dest_pipe: Direction) -> bool {
        match dest_pipe {
            Direction::Origin | Direction::NS | Direction::NW => true,
            _ => false,
        }
    }

    fn target_coordinates(&self, (i, j): (i32, i32)) -> Option<[(i32, i32); 2]> {
        match self {
            Direction::NS => Some([(i - 1, j), (i + 1, j)]),
            Direction::EW => Some([(i, j + 1), (i, j - 1)]),
            Direction::NE => Some([(i - 1, j), (i, j + 1)]),
            Direction::NW => Some([(i - 1, j), (i, j - 1)]),
            Direction::SW => Some([(i + 1, j), (i, j - 1)]),
            Direction::SE => Some([(i + 1, j), (i, j + 1)]),
            Direction::None => None,
            Direction::Origin => None, // Ideally it shouldn't be called when on origin.
        }
    }
}

fn find_any_connecting_pipe(
    (i, j): (usize, usize),
    (m, n): (usize, usize),
    matrix: &Vec<Vec<Direction>>,
) -> (usize, usize) {
    if i + 1 < m && Direction::can_accept_from_above(matrix[i + 1][j]) {
        return (i + 1, j);
    }
    if i > 0 && Direction::can_accept_from_below(matrix[i - 1][j]) {
        return (i - 1, j);
    }
    if j > 0 && Direction::can_accept_from_right(matrix[i][j - 1]) {
        return (i, j - 1);
    }
    if j + 1 < n && Direction::can_move_left(matrix[i][j + 1]) {
        return (i, j + 1);
    }
    unreachable!("should have found any path")
}

fn find_start_point(matrix: &Vec<Vec<Direction>>) -> (usize, usize) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            match matrix[i][j] {
                Direction::Origin => return (i, j),
                _ => continue,
            }
        }
    }
    unreachable!("'S' must exists in the matrix!!!")
}
