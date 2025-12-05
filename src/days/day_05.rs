use std::{
    io::{BufWriter, Write},
    ops::RangeInclusive,
};

use aoclib_rs::{prep_io, printwriteln};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 5).unwrap();
    let contents_split: Vec<_> = contents.split(|&s| s.is_empty()).collect();
    let fresh_ranges: Vec<_> = contents_split[0]
        .iter()
        .map(|r| {
            let mut r_split = r.split("-");
            let start: i64 = r_split.next().unwrap().parse().unwrap();
            let end: i64 = r_split.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect();
    let ids: Vec<i64> = contents_split[1]
        .iter()
        .map(|id| id.parse().unwrap())
        .collect();

    part1(&mut writer, &fresh_ranges, &ids);
    part2(&mut writer, fresh_ranges);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    fresh_ranges: &Vec<RangeInclusive<i64>>,
    ids: &Vec<i64>,
) {
    let mut total = 0;
    'outer: for id in ids {
        for fresh_range in fresh_ranges {
            if fresh_range.contains(id) {
                total += 1;
                continue 'outer;
            }
        }
    }
    printwriteln!(writer, "{}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, mut fresh_ranges: Vec<RangeInclusive<i64>>) {
    loop {
        let mut merged_any = false;
        let mut merged_ranges: Vec<RangeInclusive<i64>> = Vec::new();
        for fresh_range in fresh_ranges {
            if merged_ranges.is_empty() {
                merged_ranges.push(fresh_range.clone());
                continue;
            }

            let mut was_merged = false;
            for merged_range in &mut merged_ranges {
                let (new_merged_range, merged) = maybe_merge_ranges(merged_range, &fresh_range);
                if merged {
                    *merged_range = new_merged_range;
                    was_merged = true;
                    merged_any = true;
                    break;
                }
            }
            if !was_merged {
                merged_ranges.push(fresh_range.clone());
            }
        }

        fresh_ranges = merged_ranges;
        if !merged_any {
            break;
        }
    }

    let mut total = 0;
    for fresh_range in fresh_ranges {
        total += *fresh_range.end() - *fresh_range.start() + 1;
    }

    printwriteln!(writer, "{}", total).unwrap();
}

fn maybe_merge_ranges(
    merged_range: &RangeInclusive<i64>,
    fresh_range: &RangeInclusive<i64>,
) -> (RangeInclusive<i64>, bool) {
    let merged_contains_start = merged_range.contains(fresh_range.start());
    let merged_contains_end = merged_range.contains(fresh_range.end());
    let fresh_contains_start = fresh_range.contains(merged_range.start());
    let fresh_contains_end = fresh_range.contains(merged_range.end());

    if merged_contains_start && merged_contains_end {
        return (merged_range.clone(), true);
    } else if fresh_contains_start && fresh_contains_end {
        return (fresh_range.clone(), true);
    } else if merged_contains_start {
        return (*merged_range.start()..=*fresh_range.end(), true);
    } else if merged_contains_end {
        return (*fresh_range.start()..=*merged_range.end(), true);
    }

    (merged_range.clone(), false)
}
