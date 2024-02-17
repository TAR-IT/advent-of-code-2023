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
    let mut it = input.lines();
    let times = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());
    let distances = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap());
    let product: i32 = times
        .zip(distances)
        .map(|(t, d)| {
            let mp = (t as f64) / 2.0;
            let offset = ((t as f64).powi(2) / 4.0 - ((d + 1) as f64)).sqrt();
            let from = (mp - offset).ceil() as i32;
            let to = (mp + offset).floor() as i32;
            to + 1 - from
        })
        .product();
    product
}

fn part_2(input: &str) -> i64 {
    let mut it = input.lines();
    let t: String = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let d: String = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let t = t.parse::<i64>().unwrap();
    let d = d.parse::<i64>().unwrap();
    let mp = (t as f64) / 2.0;
    let offset = ((t as f64).powi(2) / 4.0 - ((d + 1) as f64)).sqrt();
    let from = (mp - offset).ceil() as i64;
    let to = (mp + offset).floor() as i64;

    to + 1 - from
}
