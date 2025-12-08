use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{BufWriter, Write},
};

use aoclib_rs::{pairwise_iter, prep_io, printwriteln};

// TODO: refactor into aoclib-rs
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3d {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    fn dist(&self, other: &Self) -> f64 {
        ((self.x as f64 - other.x as f64).powi(2)
            + (self.y as f64 - other.y as f64).powi(2)
            + (self.z as f64 - other.z as f64).powi(2))
        .sqrt()
    }
}

type Dist = (Point3d, Point3d, f64);

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 8).unwrap();
    let points: Vec<_> = contents
        .iter()
        .map(|p| {
            let mut p_split = p.split(",");
            Point3d::new(
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut dists: Vec<Dist> = Vec::new();

    // TODO: slightly inefficient - makes copies of points instead of passing by ref
    for (p1, p2) in pairwise_iter(&points) {
        dists.push((p1, p2, p1.dist(&p2)));
    }
    dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    part1(&mut writer, &dists);
    part2(&mut writer, &dists);
}

type Circuit = HashSet<Point3d>;

fn part1<W: Write>(writer: &mut BufWriter<W>, dists: &[Dist]) {
    let (circuits, _, _) = solve_for_n_pairs(dists, 1000);
    printwriteln!(
        writer,
        "{}",
        circuits[circuits.len() - 1].len()
            * circuits[circuits.len() - 2].len()
            * circuits[circuits.len() - 3].len()
    )
    .unwrap();
}

type Solution = (Vec<Circuit>, Point3d, Point3d);

fn solve_for_n_pairs(dists: &[Dist], n: usize) -> Solution {
    let mut pushed: HashSet<Point3d> = HashSet::new();
    let mut circuits: Vec<Circuit> = Vec::new();
    for d in dists.iter().take(n) {
        if !pushed.contains(&d.0) {
            circuits.push(HashSet::from([d.0]));
            pushed.insert(d.0);
        }
        if !pushed.contains(&d.1) {
            circuits.push(HashSet::from([d.1]));
            pushed.insert(d.1);
        }
    }

    let mut i = 0;
    loop {
        let i1 = find_circuit_containing(&circuits, &dists[i].0);
        let i2 = find_circuit_containing(&circuits, &dists[i].1);
        merge_circuits(&mut circuits, i1, i2);
        if i == n - 2 || circuits.len() == 1 {
            break;
        }
        i += 1;
    }

    circuits.sort_by_key(|e| e.len());

    (circuits, dists[i].0, dists[i].1)
}

fn find_circuit_containing(circuits: &[Circuit], p: &Point3d) -> usize {
    circuits.iter().position(|c| c.contains(p)).unwrap()
}

fn merge_circuits(circuits: &mut Vec<Circuit>, i1: usize, i2: usize) {
    if i1 == i2 {
        return;
    }
    let c2 = circuits.remove(max(i1, i2));
    circuits[min(i1, i2)].extend(c2);
}

fn part2<W: Write>(writer: &mut BufWriter<W>, dists: &[Dist]) {
    let (circuits, p1, p2) = solve_for_n_pairs(dists, dists.len());
    if circuits.len() != 1 {
        panic!(
            "something went wrong, still have {} circuits",
            circuits.len()
        );
    }

    printwriteln!(writer, "{}", p1.x * p2.x).unwrap();
}
