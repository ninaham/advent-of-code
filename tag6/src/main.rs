use std::fs;

fn main() {
    println!("{}", num_of_wins(parse_input(read_input())));
    println!("{}", find_wins(parse_input_part_2(read_input())))
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let mut lines = input
        .lines()
        .map(|l| l.split(':').nth(1).unwrap().trim())
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
        });

    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times.zip(distances).collect()
}

fn parse_input_part_2(input: String) -> (usize, usize) {
    let mut lines = input
        .lines()
        .map(|l| l.split(':').nth(1).unwrap().trim())
        .map(|l| {
            l.split_whitespace().collect::<String>().parse::<usize>().unwrap()
        });

    (lines.next().unwrap(), lines.next().unwrap())
}

fn num_of_wins(input: Vec<(usize, usize)>) -> usize {
    input.iter().map(|n| find_wins(*n)).product()
}

fn find_wins(v: (usize, usize)) -> usize {
    (0..v.0).map(|x| v.0 * x - x * x).filter(|y| *y > v.1).count()
}