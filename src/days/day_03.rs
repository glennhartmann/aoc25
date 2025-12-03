use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 3).unwrap();
    let banks: Vec<_> = contents
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|b| b.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    part1(&mut writer, &banks);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, banks: &Vec<Vec<u32>>) {
    let mut total = 0;
    for bank in banks {
        let first = bank[..(bank.len() - 1)].iter().max().unwrap();
        let first_index = bank.iter().position(|p| p == first).unwrap();
        let second = bank[(first_index + 1)..].iter().max().unwrap();
        let num = first * 10 + second;

        println!("{}", num);

        total += num;
    }

    printwriteln!(writer, "{}", total).unwrap();
}
