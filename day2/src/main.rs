use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt").expect("Unable to open input.txt").read_to_string(&mut input).expect("Unable to read input.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let colors: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    input.lines().enumerate().filter_map(|(n, line)| {
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            if count > colors[color] {
                return None;
            }
        }
        Some(n + 1)
    }).sum()
}

fn part_2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut colors: HashMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into_iter().collect();
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            let max = colors.get_mut(color).unwrap();
            if count > *max {
                *max = count;
            }
        }
        colors.into_iter().map(|(_, count)| count).product::<u32>()
    }).sum()
}

