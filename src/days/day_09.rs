use std::io::{BufWriter, Write};

use aoclib_rs::{pairwise_iter, prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 9).unwrap();
    let points: Vec<_> = contents
        .iter()
        .map(|p| {
            let mut p_split = p.split(",");
            (
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    part1(&mut writer, &points);
}

type Point = (i64, i64);

fn part1<W: Write>(writer: &mut BufWriter<W>, points: &[Point]) {
    let area = |(p1, p2): &(Point, Point)| ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);

    // TODO: slightly inefficient - makes copies of points instead of passing by ref
    let max_area_points = pairwise_iter(points).max_by_key(area).unwrap();
    printwriteln!(writer, "{}", area(&max_area_points)).unwrap();
}
