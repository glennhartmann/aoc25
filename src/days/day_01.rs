use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum DirDist {
    Left(i32),
    Right(i32),
}

impl From<&str> for DirDist {
    fn from(s: &str) -> Self {
        let sc: Vec<_> = s.chars().collect();
        let val: i32 = s[1..].parse().unwrap();
        match sc[0] {
            'L' => DirDist::Left(val),
            'R' => DirDist::Right(val),
            _ => panic!("invalid input"),
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 1).unwrap();
    let code: Vec<_> = contents.iter().map(|&i| DirDist::from(i)).collect();

    part1(&mut writer, &code);
    part2(&mut writer, &code);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, code: &Vec<DirDist>) {
    let mut dial = 50;
    let mut password = 0;

    for dd in code {
        dial = match dd {
            DirDist::Left(dist) => dial - dist,
            DirDist::Right(dist) => dial + dist,
        };
        while !(0..=99).contains(&dial) {
            dial = match dial {
                i32::MIN..0 => dial + 100,
                100..=i32::MAX => dial - 100,
                _ => panic!("impossible"),
            };
        }

        if dial == 0 {
            password += 1;
        }
    }

    printwriteln!(writer, "{}", password).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, code: &Vec<DirDist>) {
    let mut dial = 50;
    let mut password = 0;

    for dd in code {
        // This implementation is very low-effort and inefficient, but hey, it's day 1
        match dd {
            DirDist::Left(dist) => {
                for _ in 0..*dist {
                    dial -= 1;
                    if dial < 0 {
                        dial += 100;
                    }
                    if dial == 0 {
                        password += 1;
                    }
                }
            }
            DirDist::Right(dist) => {
                for _ in 0..*dist {
                    dial += 1;
                    if dial > 99 {
                        dial -= 100;
                    }
                    if dial == 0 {
                        password += 1;
                    }
                }
            }
        }
    }

    printwriteln!(writer, "{}", password).unwrap();
}
