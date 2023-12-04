use std::{collections::HashSet, fs};

fn main() {
    let input: Vec<String> = get_input().lines().map(|s| s.to_string()).collect();
    let mut copies = vec![1; input.len()];
    let nums: usize = input
        .iter()
        .enumerate()
        .map(|l| compare_numbers(l.0, parse_input(l.1), false, &mut copies))
        .sum();

    println!("{}", nums);

    input.iter().enumerate().for_each(|l| {
        compare_numbers(l.0, parse_input(l.1), true, &mut copies);
    });

    println!("{}", copies.iter().sum::<usize>());
}

fn get_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let relevant_part = input.split(':').nth(1).unwrap();
    let left = relevant_part
        .split('|')
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let right = relevant_part
        .split('|')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect();

    (left, right)
}

fn compare_numbers(
    i: usize,
    numbers: (Vec<usize>, Vec<usize>),
    part2: bool,
    copies: &mut [usize],
) -> usize {
    let mut first_numbers = HashSet::new();

    numbers.0.iter().for_each(|n| {
        first_numbers.insert(*n);
    });

    let count = numbers
        .1
        .iter()
        .filter(|n| first_numbers.contains(*n))
        .count();

    if part2 {
        for j in (i + 1)..(i + count + 1) {
            copies[j] += copies[i];
        }
    }

    if count == 0 {
        0
    } else {
        2_usize.pow(count as u32 - 1) * copies[i]
    }
}
