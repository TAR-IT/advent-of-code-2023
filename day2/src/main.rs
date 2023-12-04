use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    // Read the contents of the "input.txt" file into a String
    let mut input = String::new();
    File::open("input.txt").expect("Unable to open input.txt")
        .read_to_string(&mut input).expect("Unable to read input.txt");

    // Call the part_1 and part_2 functions with the input and print the results
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

// Function for solving Part 1 of the problem
fn part_1(input: &str) -> usize {
    // Define a HashMap mapping color names to their allowed counts
    let colors: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    // For each line in the input, check if the count of each color is within the allowed limits
    input.lines().enumerate().filter_map(|(n, line)| {
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            // If the count exceeds the allowed limit, return None
            if count > colors[color] {
                return None;
            }
        }
        // Return the line number + 1 for lines that meet the criteria
        Some(n + 1)
    }).sum()  // Sum up the line numbers for valid lines
}

// Function for solving Part 2 of the problem
fn part_2(input: &str) -> u32 {
    // For each line in the input, calculate the product of maximum counts for each color
    input.lines().map(|line| {
        // Initialize a HashMap to track maximum counts for each color
        let mut colors: HashMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into_iter().collect();
        let mut iter = line.split_ascii_whitespace().skip(2);
        while let Some(x) = iter.next() {
            let count: u32 = x.parse().unwrap();
            let color = iter.next().unwrap().trim_end_matches([',', ';']);
            // Update the maximum count for the current color if necessary
            let max = colors.get_mut(color).unwrap();
            if count > *max {
                *max = count;
            }
        }
        // Calculate the product of maximum counts for all colors
        colors.into_iter().map(|(_, count)| count).product::<u32>()
    }).sum()  // Sum up the calculated products for all lines
}
