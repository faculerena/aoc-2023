use crate::input_string;
use itertools::Itertools;
use std::io::Read;
#[derive(Debug, Clone)]
struct MapEntry {
    dest_range: i64,
    source_range: i64,
    range_length: i64,
}
#[derive(Debug, Clone)]
struct SeedRange {
    start: i64,
    end: i64,
}
pub fn run1() -> String {
    let input = input_string!();

    let maps: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect();

    let seeds: Vec<i64> = maps[0][0][7..]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut all_maps = Vec::new();

    for p in &maps[1..] {
        let map_entries: Vec<MapEntry> = p[1..]
            .iter()
            .map(|x| {
                let line: Vec<&str> = x.split(" ").collect();
                MapEntry {
                    dest_range: line[0].parse::<i64>().unwrap(),
                    source_range: line[1].parse::<i64>().unwrap(),
                    range_length: line[2].parse::<i64>().unwrap(),
                }
            })
            .collect();
        all_maps.push(map_entries);
    }

    let mut it = seeds.clone();

    for this_map in all_maps.iter() {
        it = it
            .iter()
            .map(|seed| {
                let found_entry = this_map.iter().find(|entry| {
                    seed >= &entry.source_range
                        && seed <= &(entry.source_range + entry.range_length)
                });

                found_entry.map_or(*seed, |entry| {
                    entry.dest_range + (seed - entry.source_range)
                })
            })
            .collect();
    }

    it.sort();
    it[0].to_string()
}

pub fn run2() -> String {
    let input = input_string!();

    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let bind_seeds = seeds_str
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2);

    let seeds = bind_seeds.into_iter().map(|mut chunk| {
        let start = chunk.next().unwrap();
        let range_length = chunk.next().unwrap();
        SeedRange {
            start,
            end: start + range_length,
        }
    });

    let maps: Vec<Vec<MapEntry>> = maps_str
        .split("\n\n")
        .map(|block| {
            block.split("\n").collect::<Vec<_>>()[1..]
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .enumerate()
                        .map(|(index, num)| num.parse().map_err(|e| (index, e)))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|nums| MapEntry {
                            dest_range: nums[0],
                            source_range: nums[1],
                            range_length: nums[2],
                        })
                        .unwrap()
                })
                .sorted_by(|a, b| a.source_range.cmp(&b.source_range))
                .collect()
        })
        .collect();

    let mut step_ranges: Vec<SeedRange> = seeds.collect();

    for map in &maps {
        let mut next_ranges: Vec<SeedRange> = Vec::new();

        for range_length in &step_ranges {
            let mut curr = range_length.clone();

            for rule in map {
                let offset = rule.dest_range - rule.source_range;

                if curr.start <= curr.end
                    && curr.start <= rule.source_range + rule.range_length
                    && curr.end >= rule.source_range
                {
                    if curr.start < rule.source_range {
                        next_ranges.push(SeedRange {
                            start: curr.start,
                            end: rule.source_range - 1,
                        });
                        curr.start = rule.source_range;

                        if curr.end < rule.source_range + rule.range_length {
                            next_ranges.push(SeedRange {
                                start: curr.start + offset,
                                end: curr.end + offset,
                            });
                            curr.start = curr.end + 1;
                        } else {
                            next_ranges.push(SeedRange {
                                start: curr.start + offset,
                                end: rule.source_range + rule.range_length - 1 + offset,
                            });
                            curr.start = rule.source_range + rule.range_length;
                        }
                    } else if curr.end < rule.source_range + rule.range_length {
                        next_ranges.push(SeedRange {
                            start: curr.start + offset,
                            end: curr.end + offset,
                        });
                        curr.start = curr.end + 1;
                    } else {
                        next_ranges.push(SeedRange {
                            start: curr.start + offset,
                            end: rule.source_range + rule.range_length - 1 + offset,
                        });
                        curr.start = rule.source_range + rule.range_length;
                    }
                }
            }
            if curr.start <= curr.end {
                next_ranges.push(curr);
            }
        }
        step_ranges = next_ranges;
    }

    step_ranges
        .iter()
        .map(|range_length| range_length.start)
        .min()
        .unwrap()
        .to_string()
}
