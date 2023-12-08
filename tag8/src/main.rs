use std::{collections::HashMap, fs};

use ::num::integer::lcm;
use scanf::sscanf;

fn main() {
    let input = parse_input(read_input());
    let starting_nodes = find_starting_nodes(&input.1);

    println!(
        "{}",
        starting_nodes
            .iter()
            .map(|s| walk_instructions(&input, s))
            .fold(1, lcm)
    );
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: String) -> (Vec<usize>, HashMap<String, Vec<String>>) {
    let mut inp = input.split("\n\n");
    let lr = inp.next().unwrap();
    let map = inp.next().unwrap();
    let mut ret = HashMap::new();

    let instructions = lr
        .trim()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("da stimmt was nicht"),
        })
        .collect::<Vec<usize>>();

    map.lines().for_each(|l| {
        let mut k: String = "Error".to_string();
        let mut left: String = "Error".to_string();
        let mut right: String = "Error".to_string();

        sscanf!(l, "{} = ({}, {})", k, left, right).unwrap();

        ret.insert(k, vec![left, right]);
    });

    (instructions, ret)
}

fn find_starting_nodes(map: &HashMap<String, Vec<String>>) -> Vec<&str> {
    map.iter()
        .filter(|(n, _)| n.chars().nth(2).unwrap() == 'A')
        .map(|(n, _)| n.as_str())
        .collect()
}

fn walk_instructions(input: &(Vec<usize>, HashMap<String, Vec<String>>), start: &str) -> usize {
    let mut counter = 0;
    let mut pos = (start, input.1.get(start).unwrap());

    while pos.0.chars().nth(2).unwrap() != 'Z' {
        let lr = input.0[counter % input.0.len()];
        pos = (pos.1[lr].as_str(), input.1.get(&pos.1[lr]).unwrap());

        counter += 1;
    }

    counter
}
