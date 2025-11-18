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
    part2(&mut writer, &graph);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, graph: &Graph) {
    let mut intermediate_paths = HashMap::new();
    let (_, _, _, paths) = dfs(graph, "you", "out", &mut intermediate_paths);
    printwriteln!(writer, "{}", paths).unwrap();
}

type PathsIfAncestorsContainNeither = i64;
type PathsIfAncestorsContainDac = i64;
type PathsIfAncestorsContainFft = i64;
type PathsIfAncestorsContainBoth = i64;

type DfsResult = (
    PathsIfAncestorsContainNeither,
    PathsIfAncestorsContainDac,
    PathsIfAncestorsContainFft,
    PathsIfAncestorsContainBoth,
);

// Example scenario (note that in the actual problem, nodes (like dac and fft) cannot be repeated
// multiple times in the same path, but this implementation also handles a more general form of the
// problem where nodes could repeat. The primes are just for disambiguating the table below).
//
// ┌───────┐
// │svr    │
// └┬──┬──┬┘
// ┌▽┐┌▽┐┌▽────────┐
// │A││B││fft      │
// └┬┘└┬┘└──┬─────┬┘
//  │┌─▽──┐┌▽───┐┌▽──────────┐
//  ││dac ││fft'││dac'       │
//  │└┬───┘└┬───┘└┬─────────┬┘
//  │ │     │┌────▽───────┐┌▽──────────┐
//  │ │     ││fft''       ││C          │
//  │ │     │└┬─────────┬─┘└┬─────────┬┘
//  │ │     │┌▽───────┐┌▽┐┌─▽───────┐┌▽┐
//  │ │     ││D       ││E││F        ││G│
//  │ │     │└┬───────┘└┬┘└┬────────┘└┬┘
//  │ │     │┌▽───────┐ │  │          │
//  │ │     ││H       │ │  │          │
//  │ │     │└┬───────┘ │  │          │
// ┌▽─▽─────▽─▽─────────▽──▽──────────▽┐
// │out                                │
// └───────────────────────────────────┘
//
// In this diagram, the nodes would output the following DfsResults:
//
// ╭─────┬─────────╮
// │svr  │(4 5 5 7)│
// ├─────┼─────────┤
// │A    │(0 0 0 1)│
// ├─────┼─────────┤
// │B    │(0 0 1 1)│
// ├─────┼─────────┤
// │fft  │(4 5 4 5)│
// ├─────┼─────────┤
// │dac  │(0 0 1 1)│
// ├─────┼─────────┤
// │fft' │(0 1 0 1)│
// ├─────┼─────────┤
// │dac' │(2 2 4 4)│
// ├─────┼─────────┤
// │fft''│(0 2 0 2)│
// ├─────┼─────────┤
// │C    │(0 0 0 2)│
// ├─────┼─────────┤
// │D    │(0 0 0 1)│
// ├─────┼─────────┤
// │E    │(0 0 0 1)│
// ├─────┼─────────┤
// │F    │(0 0 0 1)│
// ├─────┼─────────┤
// │G    │(0 0 0 1)│
// ├─────┼─────────┤
// │H    │(0 0 0 1)│
// ├─────┼─────────┤
// │out  │(0 0 0 1)│
// ╰─────┴─────────╯
//
// Thus we can see that there are 4 paths from svr to out that include at least one dac and at
// least one fft, and there are 7 paths from svr to out in total.
//
// Credit to https://diagon.arthursonzogni.com/ for the unicode diagrams.
fn dfs(graph: &Graph, start: &str, end: &str, paths: &mut HashMap<String, DfsResult>) -> DfsResult {
    if start == end {
        // because we know end != "dac" and end != "fft"
        return (0, 0, 0, 1);
    }

    if let Some(paths) = paths.get(start) {
        return *paths;
    }

    let (
        mut total_if_ancestors_contain_neither,
        mut total_if_ancestors_contain_dac,
        mut total_if_ancestors_contain_fft,
        mut total_if_ancestors_contain_both,
    ) = (0, 0, 0, 0);
    for output in graph.get(start).unwrap() {
        let (
            paths_if_ancestors_contain_neither,
            paths_if_ancestors_contain_dac,
            paths_if_ancestors_contain_fft,
            paths_if_ancestors_contain_both,
        ) = dfs(graph, output, end, paths);

        total_if_ancestors_contain_neither += if start == "dac" {
            paths_if_ancestors_contain_dac
        } else if start == "fft" {
            paths_if_ancestors_contain_fft
        } else {
            paths_if_ancestors_contain_neither
        };

        total_if_ancestors_contain_dac += paths_if_ancestors_contain_dac;
        total_if_ancestors_contain_fft += paths_if_ancestors_contain_fft;

        total_if_ancestors_contain_both += paths_if_ancestors_contain_both;
    }

    if start == "dac" {
        total_if_ancestors_contain_fft = total_if_ancestors_contain_both;
    } else if start == "fft" {
        total_if_ancestors_contain_dac = total_if_ancestors_contain_both;
    }

    let result = (
        total_if_ancestors_contain_neither,
        total_if_ancestors_contain_dac,
        total_if_ancestors_contain_fft,
        total_if_ancestors_contain_both,
    );

    paths.insert(start.to_owned(), result);
    result
}

fn part2<W: Write>(writer: &mut BufWriter<W>, graph: &Graph) {
    let mut intermediate_paths = HashMap::new();
    let (paths, _, _, _) = dfs(graph, "svr", "out", &mut intermediate_paths);
    printwriteln!(writer, "{}", paths).unwrap();
}
