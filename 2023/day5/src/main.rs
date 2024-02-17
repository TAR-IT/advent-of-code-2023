use itertools::Itertools;
use std::fs::File;
use std::io::{Read, Error};

fn main() {
    if let Err(e) = solve() {
        eprintln!("Error: {}", e);
    }
}

fn solve() -> Result<(), Error> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

fn part_1(input: &str) -> i64 {
    let mut groups = input.split("\n\n");
    let seed_group = groups.next().unwrap();
    let mut values: Vec<_> = seed_group
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for group in groups {
        let mappings: Vec<(i64, i64, i64)> = group
            .lines()
            .skip(1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        values = values
            .into_iter()
            .map(|value| {
                if let Some(m) = mappings.iter().find(|m| value >= m.1 && value < m.1 + m.2) {
                    value - m.1 + m.0
                } else {
                    value
                }
            })
            .collect();
    }
    values.into_iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let mut groups = input.split("\n\n");
    let seed_group = groups.next().unwrap();
    let mut values: Vec<(i64, i64)> = seed_group
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
        .collect();
    for group in groups {
        let mappings: Vec<_> = group
            .lines()
            .skip(1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
                    .unwrap()
            })
            .sorted_by_key(|m| m.1)
            .collect();

        values = values
            .into_iter()
            .flat_map(|value| {
                let mut result = Vec::new();
                let mut current = value.0;
                let end = value.0 + value.1;
                for m in &mappings {
                    if m.1 > current {
                        result.push((current, m.1 - current));
                        current = m.1;
                    }
                    if current >= end {
                        break;
                    }
                    let to = end.min(m.1 + m.2);
                    if to > current {
                        result.push((current - m.1 + m.0, to - current));
                        current = to;
                    }
                    if current >= end {
                        break;
                    }
                }
                result
            })
            .collect();
    }
    values.into_iter().map(|x| x.0).min().unwrap()
}
