use std::io::{BufWriter, Write};

use aoclib_rs::{iter::pairwise_iter, point::Point2d, prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 9).unwrap();
    let points: Vec<_> = contents
        .iter()
        .map(|p| {
            let mut p_split = p.split(",");
            P2::new(
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    part1(&mut writer, &points);
}

type P2 = Point2d<i64>;

fn part1<W: Write>(writer: &mut BufWriter<W>, points: &[P2]) {
    let area =
        |(p1, p2): &(&P2, &P2)| ((p1.x() - p2.x()).abs() + 1) * ((p1.y() - p2.y()).abs() + 1);

    let max_area_points = pairwise_iter(points).max_by_key(area).unwrap();
    printwriteln!(writer, "{}", area(&max_area_points)).unwrap();
}
