use std::io::{BufWriter, Write};

use aoclib_rs::{
    dir::{Dir8, Direction},
    prep_io, printwriteln,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Cell {
    Empty,
    PaperRoll,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::PaperRoll,
            _ => panic!("invalid input"),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 4).unwrap();
    let map: Vec<Vec<Cell>> = contents
        .into_iter()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect();
    let mut map: Vec<Vec<Cell>> = pad(&map, 1, Cell::Empty);

    part1(&mut writer, &map);
    part2(&mut writer, &mut map);
}

// TODO: refactor into aoclib_rs::pad
pub fn pad<T: Clone + Copy>(contents: &Vec<Vec<T>>, padding: usize, default: T) -> Vec<Vec<T>> {
    let mut r = Vec::with_capacity(contents.len());
    let mut prefix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut prefix);

    for line in contents {
        let prefix = vec![default; padding];
        let middle = line.to_vec();
        let suffix = vec![default; padding];

        r.push(vec![prefix, middle, suffix].into_iter().flatten().collect());
    }

    let mut suffix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut suffix);

    r
}

fn part1<W: Write>(writer: &mut BufWriter<W>, map: &[Vec<Cell>]) {
    let mut total = 0;
    for row in 1..(map.len() - 1) {
        for col in 1..(map[row].len() - 1) {
            if map[row][col] == Cell::PaperRoll && can_forklift(map, row, col) {
                total += 1;
            }
        }
    }
    printwriteln!(writer, "{}", total).unwrap();
}

fn can_forklift(map: &[Vec<Cell>], row: usize, col: usize) -> bool {
    let mut total = 0;
    for d in Dir8::iter() {
        let adjacent = d.apply_delta_to_usizes((row, col));
        if map[adjacent.0][adjacent.1] == Cell::PaperRoll {
            total += 1;
        }
    }
    total < 4
}

fn part2<W: Write>(writer: &mut BufWriter<W>, map: &mut [Vec<Cell>]) {
    let mut total = 0;
    loop {
        let mut total_this_round = 0;
        for row in 1..(map.len() - 1) {
            for col in 1..(map[row].len() - 1) {
                if map[row][col] == Cell::PaperRoll && can_forklift(map, row, col) {
                    total_this_round += 1;
                    total += 1;
                    map[row][col] = Cell::Empty;
                }
            }
        }
        if total_this_round == 0 {
            break;
        }
    }
    printwriteln!(writer, "{}", total).unwrap();
}
