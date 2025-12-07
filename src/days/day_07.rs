use std::{
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
};

use aoclib_rs::{
    dir::{Dir4, Direction},
    position_2d, prep_io, printwriteln,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            _ => panic!("invalid input"),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 7).unwrap();
    let map: Vec<Vec<_>> = contents
        .iter()
        .map(|&row| row.chars().map(Cell::from).collect())
        .collect();
    let start_pos = position_2d(&map, |&cell| cell == Cell::Start).unwrap();

    part1(&mut writer, &map, start_pos);
    part2(&mut writer, &map, start_pos);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, map: &[Vec<Cell>], start_pos: (usize, usize)) {
    let mut splits = 0;

    let mut beams: HashSet<(usize, usize)> = HashSet::new();
    beams.insert(start_pos);
    'outer: loop {
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
        for mut beam in beams {
            beam = Dir4::Down.apply_delta_to_usizes(beam);
            if beam.1 >= map.len() {
                break 'outer;
            }
            if map[beam.1][beam.0] == Cell::Splitter {
                let b1 = Dir4::Left.apply_delta_to_usizes(beam);
                let b2 = Dir4::Right.apply_delta_to_usizes(beam);
                new_beams.insert(b1);
                new_beams.insert(b2);
                splits += 1;
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }

    printwriteln!(writer, "{}", splits).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, map: &[Vec<Cell>], start_pos: (usize, usize)) {
    let mut beams: HashMap<(usize, usize), i64> = HashMap::new();
    beams.insert(start_pos, 1);
    'outer: loop {
        let mut new_beams: HashMap<(usize, usize), i64> = HashMap::new();
        for (beam, timelines) in &beams {
            let beam = Dir4::Down.apply_delta_to_usizes(*beam);
            if beam.1 >= map.len() {
                break 'outer;
            }
            if map[beam.1][beam.0] == Cell::Splitter {
                let b1 = Dir4::Left.apply_delta_to_usizes(beam);
                let b2 = Dir4::Right.apply_delta_to_usizes(beam);
                new_beams
                    .entry(b1)
                    .and_modify(|e| *e += *timelines)
                    .or_insert(*timelines);
                new_beams
                    .entry(b2)
                    .and_modify(|e| *e += *timelines)
                    .or_insert(*timelines);
            } else {
                new_beams
                    .entry(beam)
                    .and_modify(|e| *e += *timelines)
                    .or_insert(*timelines);
            }
        }
        beams = new_beams;
    }

    let timelines: i64 = beams.values().sum();
    printwriteln!(writer, "{}", timelines).unwrap();
}
