use std::collections::HashMap;
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

fn part_1(input: &str) -> usize {
    let colors: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    input.lines()
        .enumerate()
        .filter_map(|(n, line)| {
            let mut iter = line.split_ascii_whitespace().skip(2);
            while let Some(x) = iter.next() {
                let count: u32 = x.parse().unwrap();
                let color = iter.next().unwrap().trim_end_matches([',', ';']);
                if count > *colors.get(color).unwrap_or(&0) as u32 {
                    return None;
                }
            }
            Some(n + 1)
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let mut colors: HashMap<String, _> = [("red".to_string(), 0), ("green".to_string(), 0), ("blue".to_string(), 0)]
                .iter()
                .cloned()
                .collect();
            
            let mut iter = line.split_ascii_whitespace().skip(2);
            while let Some(x) = iter.next() {
                let count: u32 = x.parse().unwrap();
                let color = iter.next().unwrap().trim_end_matches([',', ';']).to_string();
                if let Some(max) = colors.get_mut(&color) {
                    if count > *max {
                        *max = count;
                    }
                } else {
                    colors.insert(color, count);
                }
            }
            
            colors.into_iter().map(|(_, count)| count).product::<u32>()
        })
        .sum()
}

