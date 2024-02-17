use std::fs::File;
use std::io::{Read, Error};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

static CARDS: &str = "23456789TJQKA";

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
    let sum: i64 = input.lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: i64 = bid.parse().unwrap();
            let hand: Vec<usize> = hand.chars().map(|c| CARDS.find(c).unwrap()).collect();
            let counts: Vec<_> = hand.iter().copied().counts().into_values().sorted().collect();
            let hand_kind = match counts.as_slice() {
                [1, 1, 1, 1, 1] => HandKind::HighCard,
                [1, 1, 1, 2] => HandKind::OnePair,
                [1, 2, 2] => HandKind::TwoPair,
                [1, 1, 3] => HandKind::ThreeOfAKind,
                [2, 3] => HandKind::FullHouse,
                [1, 4] => HandKind::FourOfAKind,
                [5] => HandKind::FiveOfAKind,
                _ => unreachable!(),
            };
            (hand_kind, hand, bid)
        })
        .sorted()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (rank as i64 + 1))
        .sum();

    sum as usize
}

fn part_2(input: &str) -> u32 {
    let sum: u32 = input.lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid: u32 = bid.parse().unwrap();
            let (mut ranks, mut power, mut jokers) = ([0u8; 13], 0, 0);
            for i in 0..5 {
                let card = match hand.as_bytes()[i] {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    n => n - b'0' - 1,
                };
                ranks[card as usize] += 1 * (card != 0) as u8;
                power |= (card as u32) << 4 * (4 - i);
                jokers += 1 * (card == 0) as u8;
            }
            ranks.sort_unstable_by(|a, b| b.cmp(a));
            power |= match ranks[0] + jokers {
                5 => 6,
                4 => 5,
                3 if ranks[1] == 2 => 4,
                3 => 3,
                2 if ranks[1] == 2 => 2,
                2 => 1,
                _ => 0,
            } << 29;
            (power, bid)
        })
        .sorted()
        .enumerate()
        .map(|(i, (_power, bet))| bet * (i as u32 + 1))
        .sum();

    sum
}