use std::{fs, io::Error};

fn main() {
    println!(
        "{}",
        parse_input()
            .unwrap()
            .lines()
            .map(|l| find_numbers(l.to_string()))
            .map(|num| num.parse::<usize>().unwrap())
            .sum::<usize>()
    );
}

fn parse_input() -> Result<String, Error> {
    fs::read_to_string("./input.txt")
}

fn _find_numbers_part_1(input: String) -> String {
    let mut nums = input.chars().filter(|c| c.is_ascii_digit());
    let first = nums.next().unwrap();
    let last = nums.last().unwrap_or(first);

    [first, last].iter().collect()
}

fn find_numbers(input: String) -> String {
    let substrings = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut indices: Vec<(usize, &str)> = substrings
        .iter()
        .flat_map(|ss| input.match_indices(*ss).collect::<Vec<(usize, &str)>>())
        .map(|(i, ss)| {
            (
                i,
                match ss {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    x => x,
                },
            )
        })
        .collect();

    indices.sort_by(|a, b| a.0.cmp(&b.0));

    let first = indices.first().unwrap().1;
    let last = indices.iter().last().unwrap().1;

    [first, last].iter().copied().collect()
}
