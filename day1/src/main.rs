use std::fs::File;
use std::io::Read;


fn part_1(input: &str) -> u32 {
    input.lines().map(|line| {
        line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10 +
        line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
    }).sum()
}

const DIGITS: [[&str; 2]; 9] = [
    ["one", "1"], ["two", "2"], ["three", "3"], ["four", "4"], ["five", "5"], ["six", "6"], ["seven", "7"],
    ["eight", "8"], ["nine", "9"],
];


fn part_2(input: &str) -> usize {
    input.lines().map(|line| {
        DIGITS.iter().enumerate().flat_map(|(n, &digits)| {
            digits.into_iter().filter_map(move |digit| line.find(digit).map(|pos| (n + 1, pos)))
        }).min_by_key(|&(_, pos)| pos).unwrap().0 * 10 +
        DIGITS.iter().enumerate().flat_map(|(n, &digits)| {
            digits.into_iter().filter_map(move |digit| line.rfind(digit).map(|pos| (n + 1, pos)))
        }).max_by_key(|&(_, pos)| pos).unwrap().0
    }).sum()
}


fn main() {
    // Open the file in read-only mode
    let file = File::open("input.txt");

    // Handle the result of opening the file
    match file {
        Ok(mut file) => {
            // Read the contents of the file into a String
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();

            // Call the part_1 and part_2 functions with the input and print the results
            println!("Part 1: {}", part_1(&input));
            println!("Part 2: {}", part_2(&input));
        }
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
        }
    }
}
