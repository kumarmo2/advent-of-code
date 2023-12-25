#![allow(dead_code, unused_mut, unused_variables)]

use std::{collections::HashMap, fmt::Display};
fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("{}", part1(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Tile {
    Space(usize, usize),
    ForwardTiltedMirror(usize, usize),
    BackwardTiltedMirror(usize, usize),
    VerticalSplitter(usize, usize),
    HorizontalSplitter(usize, usize),
}

impl Tile {
    fn get_coordinates(&self) -> (usize, usize) {
        match self {
            Tile::Space(x, y) => (*x, *y),
            Tile::ForwardTiltedMirror(x, y) => (*x, *y),
            Tile::BackwardTiltedMirror(x, y) => (*x, *y),
            Tile::VerticalSplitter(x, y) => (*x, *y),
            Tile::HorizontalSplitter(x, y) => (*x, *y),
        }
    }
    fn get_out_direction(&self, in_direction: &Direction) -> (Direction, Option<Direction>) {
        match (self, in_direction) {
            (Tile::Space(_, _), Direction::Left) => (Direction::Right, None),
            (Tile::Space(_, _), Direction::Right) => (Direction::Left, None),
            (Tile::Space(_, _), Direction::Up) => (Direction::Down, None),
            (Tile::Space(_, _), Direction::Down) => (Direction::Up, None),
            (Tile::ForwardTiltedMirror(_, _), Direction::Up) => (Direction::Left, None),
            (Tile::ForwardTiltedMirror(_, _), Direction::Down) => (Direction::Right, None),
            (Tile::ForwardTiltedMirror(_, _), Direction::Left) => (Direction::Up, None),
            (Tile::ForwardTiltedMirror(_, _), Direction::Right) => (Direction::Down, None),
            (Tile::BackwardTiltedMirror(_, _), Direction::Up) => (Direction::Right, None),
            (Tile::BackwardTiltedMirror(_, _), Direction::Down) => (Direction::Left, None),
            (Tile::BackwardTiltedMirror(_, _), Direction::Left) => (Direction::Down, None),
            (Tile::BackwardTiltedMirror(_, _), Direction::Right) => (Direction::Up, None),
            (Tile::VerticalSplitter(_, _), Direction::Up) => (Direction::Down, None),
            (Tile::VerticalSplitter(_, _), Direction::Down) => (Direction::Up, None),
            (Tile::VerticalSplitter(_, _), Direction::Left) => {
                (Direction::Up, Some(Direction::Down))
            }
            (Tile::VerticalSplitter(_, _), Direction::Right) => {
                (Direction::Up, Some(Direction::Down))
            }
            (Tile::HorizontalSplitter(_, _), Direction::Up) => {
                (Direction::Left, Some(Direction::Right))
            }
            (Tile::HorizontalSplitter(_, _), Direction::Down) => {
                (Direction::Left, Some(Direction::Right))
            }
            (Tile::HorizontalSplitter(_, _), Direction::Left) => (Direction::Right, None),
            (Tile::HorizontalSplitter(_, _), Direction::Right) => (Direction::Left, None),
        }
    }
}

fn get_next_tile(tile: Tile, out_direction: Direction, matrix: &Vec<Vec<Tile>>) -> Option<Tile> {
    let (x, y) = tile.get_coordinates();
    match out_direction {
        Direction::Up => {
            if x == 0 {
                return None;
            }
            return Some(matrix[x - 1][y].clone());
        }
        Direction::Down => {
            if x == matrix.len() - 1 {
                return None;
            }
            return Some(matrix[x + 1][y].clone());
        }
        Direction::Left => {
            if y == 0 {
                return None;
            }
            return Some(matrix[x][y - 1].clone());
        }
        Direction::Right => {
            if y == matrix[x].len() - 1 {
                return None;
            }
            return Some(matrix[x][y + 1].clone());
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug, Eq, Hash)]
struct TileDirection {
    tile: Tile,
    direction: Direction,
}

fn part1(input: &str) -> u32 {
    let mut map: HashMap<TileDirection, bool> = HashMap::new(); // This will basically
                                                                // tell you if at a point(x,y) and the light coming from the direction has already been visited
                                                                // or not.
    let mut matrix: Vec<Vec<Tile>> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        matrix.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Space(row, col),
                '/' => Tile::ForwardTiltedMirror(row, col),
                '\\' => Tile::BackwardTiltedMirror(row, col),
                '-' => Tile::HorizontalSplitter(row, col),
                '|' => Tile::VerticalSplitter(row, col),
                _ => unreachable!(),
            };
            matrix[row].push(tile);
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Left;
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    visit(&mut map, &matrix, &mut visited, &matrix[x][y], direction);
    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if visited[i][j] {
                count += 1;
            }
        }
    }
    count
}

impl PartialEq for TileDirection {
    fn eq(&self, other: &Self) -> bool {
        self.tile == other.tile && self.direction == other.direction
    }
}

fn visit(
    map: &mut HashMap<TileDirection, bool>,
    matrix: &Vec<Vec<Tile>>,
    visited: &mut Vec<Vec<bool>>,
    tile: &Tile,
    direction: Direction,
) {
    let tile_direction = TileDirection {
        tile: tile.clone(),
        direction: direction.clone(),
    };
    match map.get(&tile_direction) {
        Some(true) => return,
        _ => (),
    }
    let (x, y) = tile.get_coordinates();
    visited[x][y] = true;
    // println!("made true for (x, y) = ({}, {})", x, y);
    map.insert(tile_direction, true);
    let (first_direction, second_direction) = tile.get_out_direction(&direction);
    // println!("for tile: {:?}, and in_direction: {:?}, first_out_direction: {:?}, second_out_direction: {:?}", tile, direction, first_direction, second_direction);
    if let Some(next_tile) = get_next_tile(tile.clone(), first_direction.clone(), matrix) {
        visit(map, matrix, visited, &next_tile, first_direction.invert());
    }
    if let Some(second_direction) = second_direction {
        if let Some(next_tile) = get_next_tile(tile.clone(), second_direction.clone(), matrix) {
            visit(map, matrix, visited, &next_tile, second_direction.invert());
        }
    }
}
