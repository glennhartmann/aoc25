use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s.chars().next().unwrap() {
            '+' => Op::Add,
            '*' => Op::Multiply,
            _ => panic!("invalid input"),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 6).unwrap();

    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();
    for (i, line) in contents.iter().enumerate() {
        let line_split = line.split(" ");
        let line_split = line_split.filter(|l| !l.is_empty());

        if i < contents.len() - 1 {
            let mut nv: Vec<i64> = Vec::new();
            nv.extend(line_split.map(|i| i.parse::<i64>().unwrap()));
            nums.push(nv);
        } else {
            ops.extend(line_split.map(Op::from));
        }
    }

    part1(&mut writer, &nums, &ops);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, nums: &Vec<Vec<i64>>, ops: &[Op]) {
    let mut total = 0;
    for i in 0..nums[0].len() {
        let mut subtotal = if ops[i] == Op::Add { 0 } else { 1 };
        for row in nums {
            if ops[i] == Op::Add {
                subtotal += row[i];
            } else {
                subtotal *= row[i];
            }
        }
        total += subtotal;
    }

    printwriteln!(writer, "{}", total).unwrap();
}
