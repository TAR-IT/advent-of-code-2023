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
    // Implement part 1 logic here
    0 // Placeholder, replace with actual logic
}

fn part_2(input: &str) -> usize {
    // Implement part 2 logic here
    0 // Placeholder, replace with actual logic
}
