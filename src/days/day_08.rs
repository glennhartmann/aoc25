use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{BufWriter, Write},
};

use aoclib_rs::{
    iter::pairwise_iter,
    point::{PointDist, Point3d},
    prep_io, printwriteln,
};

type P3 = Point3d<i64>;
type Dist = (P3, P3, f64);

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 8).unwrap();
    let points: Vec<_> = contents
        .iter()
        .map(|p| {
            let mut p_split = p.split(",");
            P3::new(
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
                p_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut dists: Vec<Dist> = Vec::new();

    for (p1, p2) in pairwise_iter(&points) {
        dists.push((p1.clone(), p2.clone(), p1.dist(p2)));
    }
    dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    part1(&mut writer, &dists);
    part2(&mut writer, &dists);
}

type Circuit = HashSet<P3>;

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

type Solution = (Vec<Circuit>, P3, P3);

fn solve_for_n_pairs(dists: &[Dist], n: usize) -> Solution {
    let mut pushed: HashSet<P3> = HashSet::new();
    let mut circuits: Vec<Circuit> = Vec::new();
    for d in dists.iter().take(n) {
        if !pushed.contains(&d.0) {
            circuits.push(HashSet::from([d.0.clone()]));
            pushed.insert(d.0.clone());
        }
        if !pushed.contains(&d.1) {
            circuits.push(HashSet::from([d.1.clone()]));
            pushed.insert(d.1.clone());
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

    (circuits, dists[i].0.clone(), dists[i].1.clone())
}

fn find_circuit_containing(circuits: &[Circuit], p: &P3) -> usize {
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

    printwriteln!(writer, "{}", p1.x() * p2.x()).unwrap();
}
