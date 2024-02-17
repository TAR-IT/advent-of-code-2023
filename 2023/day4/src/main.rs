use std::fs::File;
use std::io::{Read, Error};
use std::collections::{HashSet, VecDeque};

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
    input.lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let (_, winning) = parts.next().unwrap().split_once(": ").unwrap();
            let actual = parts.next().unwrap();
            
            let winning_numbers: HashSet<i32> = winning
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            
            let count = actual
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .count();
            
            if count > 0 {
                1 << (count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let mut sum = 0;
    let mut following = VecDeque::new();
    following.resize(input.lines().count(), 1);
    for line in input.lines() {
        let mut parts = line.split(" | ");
        let (_, winning) = parts.next().unwrap().split_once(": ").unwrap();
        let actual = parts.next().unwrap();
        
        let num_current = following.pop_front().unwrap();
        sum += num_current;
        
        let winning_numbers: HashSet<i32> = winning
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let count = actual
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count();
        for i in 0..count {
            following[i] += num_current;
        }
    }
    sum
}
