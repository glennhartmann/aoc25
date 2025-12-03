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
    part2(&mut writer, &banks);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, banks: &Vec<Vec<u32>>) {
    let total = max_for_length(banks, 2);
    printwriteln!(writer, "{}", total).unwrap();
}

fn max_for_length(banks: &Vec<Vec<u32>>, length: usize) -> u64 {
    let mut total = 0;
    for bank in banks {
        let mut num = "".to_string();
        let mut start_index = 0;
        for i in 0..length {
            let end_index = bank.len() - length + 1 + i;
            let digit = bank[start_index..end_index].iter().max().unwrap();
            let digit_index = bank[start_index..end_index]
                .iter()
                .position(|p| p == digit)
                .unwrap()
                + start_index;
            start_index = digit_index + 1;
            num = num + &format!("{}", digit);
        }

        println!("{}", num);

        total += num.parse::<u64>().unwrap();
    }
    total
}

fn part2<W: Write>(writer: &mut BufWriter<W>, banks: &Vec<Vec<u32>>) {
    let total = max_for_length(banks, 12);
    printwriteln!(writer, "{}", total).unwrap();
}
