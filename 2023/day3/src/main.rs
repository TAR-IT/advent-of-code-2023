use std::collections::{HashSet, HashMap};
use regex::Regex;
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

fn part_1(input: &str) -> i32 {
    let number_re = Regex::new("[0-9]+").unwrap();
    let symbol_re = Regex::new("[^0-9.]").unwrap();
    let symbol_positions: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            symbol_re
                .find_iter(line)
                .map(move |m| (y as i32, m.start() as i32))
        })
        .collect();
    let sum: i32 = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            number_re.find_iter(line).map(move |m| {
                (
                    y as i32,
                    m.start() as i32,
                    m.end() as i32,
                    m.as_str().parse::<i32>().unwrap(),
                )
            })
        })
        .filter(|&(y, x0, x1, _)| {
            (y - 1..=y + 1)
                .flat_map(|cy| (x0 - 1..x1 + 1).map(move |cx| (cy, cx)))
                .any(|p| symbol_positions.contains(&p))
        })
        .map(|(_, _, _, v)| v)
        .sum();
    sum
}

fn part_2(input: &str) -> i32 {
    let number_re = Regex::new("[0-9]+").unwrap();
    let mut gears: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.match_indices('*')
                .map(move |(x, _)| ((y as i32, x as i32), Vec::new()))
        })
        .collect();
    for (y, x0, x1, v) in input.lines().enumerate().flat_map(|(y, line)| {
        number_re.find_iter(line).map(move |m| {
            (
                y as i32,
                m.start() as i32,
                m.end() as i32,
                m.as_str().parse::<i32>().unwrap(),
            )
        })
    }) {
        for p in (y - 1..=y + 1).flat_map(|cy| (x0 - 1..x1 + 1).map(move |cx| (cy, cx))) {
            if let Some(vec) = gears.get_mut(&p) {
                vec.push(v);
            }
        }
    }
    let sum: i32 = gears
        .values()
        .filter(|vec| vec.len() == 2)
        .map(|vec| vec.iter().copied().product::<i32>())
        .sum();
    sum
}
