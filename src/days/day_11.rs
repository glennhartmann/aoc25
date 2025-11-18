use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

type Device = String;
type Output = String;
type Outputs = Vec<Output>;
type Graph = HashMap<Device, Outputs>;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 11).unwrap();

    let graph: Graph = contents
        .iter()
        .map(|line| {
            let mut line_split = line.split(": ");
            let device = line_split.next().unwrap();

            let outputs_str = line_split.next().unwrap();
            let outputs_split = outputs_str.split(" ");
            let outputs: Outputs = outputs_split.map(|o| o.to_owned()).collect();

            (device.to_owned(), outputs)
        })
        .collect();

    part1(&mut writer, &graph);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, graph: &Graph) {
    let mut intermediate_paths = HashMap::new();
    let paths = dfs(graph, "you", "out", &mut intermediate_paths);
    printwriteln!(writer, "{}", paths).unwrap();
}

fn dfs(graph: &Graph, start: &str, end: &str, paths: &mut HashMap<String, i64>) -> i64 {
    if start == end {
        return 1;
    }

    if let Some(paths) = paths.get(start) {
        return *paths;
    }

    let mut total = 0;
    for output in graph.get(start).unwrap() {
        total += dfs(graph, output, end, paths);
    }

    paths.insert(start.to_owned(), total);
    total
}
