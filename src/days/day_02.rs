use std::io::{BufWriter, Write};

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
}

fn part1<W: Write>(writer: &mut BufWriter<W>, ranges: &Vec<(String, String)>) {
    let mut total: i64 = 0;
    for range in ranges {
        let starti = range.0.parse().unwrap();
        let endi = range.1.parse().unwrap();
        let mut current = range.0.clone();
        while leq(&current, endi) {
            if current.len() % 2 != 0 {
                current = next_order_of_magnitude(&current);
            }

            let half = &current[..(current.len() / 2)];
            let rep = half.to_string() + half;
            if geq(&rep, starti) && leq(&rep, endi) {
                println!("{}", &rep);
                total += rep.parse::<i64>().unwrap();
            }
            let next_half = increment(half);
            current = next_half.clone() + &next_half;
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

fn next_order_of_magnitude(s: &str) -> String {
    let mut next = vec!['0'; s.len() + 1];
    next[0] = '1';
    next.into_iter().collect()
}
