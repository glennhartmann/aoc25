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

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&str>) {
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

    let total = solve(&nums, &ops);
    printwriteln!(writer, "{}", total).unwrap();
}

fn solve(nums: &Vec<Vec<i64>>, ops: &[Op]) -> i64 {
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
    total
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[&str]) {
    let num_chars: Vec<Vec<char>> = contents[..(contents.len() - 1)]
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut ops: Vec<Op> = Vec::new();
    let op_line_split = contents[contents.len() - 1].split(" ");
    let op_line_split = op_line_split.filter(|l| !l.is_empty());
    ops.extend(op_line_split.map(Op::from));

    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut start_new_problem = true;
    for col in 0..num_chars[0].len() {
        if col_is_empty(&num_chars, col) {
            start_new_problem = true;
            continue;
        }

        let mut num_str = "".to_string();
        for line in &num_chars {
            if line[col] != ' ' {
                num_str = num_str + &line[col].to_string();
            }
        }
        let num: i64 = num_str.parse().unwrap();
        if start_new_problem {
            nums.push(Vec::new());
            start_new_problem = false;
        }
        let last = nums.len() - 1;
        nums[last].push(num);
    }
    println!("{:?}", nums);

    if nums.len() != ops.len() {
        panic!(
            "length mismatch: {} groups of nums, {} ops",
            nums.len(),
            ops.len()
        );
    }

    let total = solve2(&nums, &ops);
    printwriteln!(writer, "{}", total).unwrap();
}

fn col_is_empty(num_chars: &Vec<Vec<char>>, col: usize) -> bool {
    for line in num_chars {
        if line[col] != ' ' {
            return false;
        }
    }
    true
}

fn solve2(nums: &[Vec<i64>], ops: &[Op]) -> i64 {
    let mut total = 0;
    for (i, ns) in nums.iter().enumerate() {
        let mut subtotal = if ops[i] == Op::Add { 0 } else { 1 };
        for n in ns {
            if ops[i] == Op::Add {
                subtotal += n;
            } else {
                subtotal *= n;
            }
        }
        total += subtotal;
    }
    total
}
