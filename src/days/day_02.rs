use std::{
    collections::HashSet,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 2).unwrap();

    let ranges_unsplit = contents[0].split(",");
    let mut ranges: Vec<(String, String)> = Vec::new();
    for range_unsplit in ranges_unsplit {
        let mut range_str_split = range_unsplit.split("-");
        ranges.push((
            range_str_split.next().unwrap().to_string(),
            range_str_split.next().unwrap().to_string(),
        ));
    }

    println!("{:?}", ranges);

    part1(&mut writer, &ranges);
    part2(&mut writer, &ranges);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, ranges: &Vec<(String, String)>) {
    let mut seen: HashSet<String> = HashSet::new();
    let mut total: i64 = 0;
    for range in ranges {
        let starti = range.0.parse().unwrap();
        let endi = range.1.parse().unwrap();
        let mut current = range.0.clone();
        while leq(&current, endi) {
            total += generate_patterns(2, &current, starti, endi, &mut seen);
            current = increment(&current);
        }
    }

    printwriteln!(writer, "{}", total).unwrap();
}

fn increment(s: &str) -> String {
    format!("{}", s.parse::<i64>().unwrap() + 1)
}

fn leq(start: &str, endi: i64) -> bool {
    start.parse::<i64>().unwrap() <= endi
}

fn geq(start: &str, starti: i64) -> bool {
    start.parse::<i64>().unwrap() >= starti
}

fn part2<W: Write>(writer: &mut BufWriter<W>, ranges: &Vec<(String, String)>) {
    let mut seen: HashSet<String> = HashSet::new();
    let mut total: i64 = 0;
    for range in ranges {
        let starti = range.0.parse().unwrap();
        let endi = range.1.parse().unwrap();
        let mut current = range.0.clone();
        while leq(&current, endi) {
            for p in 2..=current.len() {
                total += generate_patterns(p, &current, starti, endi, &mut seen);
            }
            current = increment(&current);
        }
    }

    printwriteln!(writer, "{}", total).unwrap();
}

fn generate_patterns(
    divisor: usize,
    current: &str,
    starti: i64,
    endi: i64,
    seen: &mut HashSet<String>,
) -> i64 {
    if current.len() % divisor != 0 {
        return 0;
    }
    let part = current[..(current.len() / divisor)].to_string();
    let mut rep = part.clone();
    for _ in 0..(divisor - 1) {
        rep = rep + &part;
    }
    let mut total = 0;
    if geq(&rep, starti) && leq(&rep, endi) && !seen.contains(&rep) {
        println!("{}", &rep);
        total += rep.parse::<i64>().unwrap();
        seen.insert(rep);
    }
    total
}
