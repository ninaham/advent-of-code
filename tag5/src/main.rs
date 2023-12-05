use std::{collections::HashMap, fs, ops::Range};

fn main() {
    let input = read_input();
    let (seeds, mut maps) = parse_input(input);

    println!(
        "{:?}",
        seeds
            .iter()
            .map(|r| {
                let ret = r.clone().map(|n| translate_seed(n, &mut maps)).min();
                println!("{:?}", r);
                ret
            })
            .min()
    )
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: String) -> (Vec<Range<usize>>, Vec<HashMap<Range<usize>, Range<usize>>>) {
    let mut seeds = Vec::new();
    let mut maps = vec![HashMap::new(); 7];
    input
        .split("\n\n")
        .for_each(|m| build_map(m.to_string(), &mut seeds, &mut maps));

    (seeds, maps)
}

fn build_map(
    input: String,
    seeds: &mut Vec<Range<usize>>,
    maps: &mut Vec<HashMap<Range<usize>, Range<usize>>>,
) {
    let mut map_split = input.split(':');
    let title = map_split.next().unwrap();
    let content = map_split.next().unwrap().trim();

    match title.split_whitespace().collect::<Vec<&str>>().as_slice() {
        &["seeds"] => parse_seeds(content, seeds),
        &["seed-to-soil", _] => parse_map(content, &mut maps[0]),
        &["soil-to-fertilizer", _] => parse_map(content, &mut maps[1]),
        &["fertilizer-to-water", _] => parse_map(content, &mut maps[2]),
        &["water-to-light", _] => parse_map(content, &mut maps[3]),
        &["light-to-temperature", _] => parse_map(content, &mut maps[4]),
        &["temperature-to-humidity", _] => parse_map(content, &mut maps[5]),
        &["humidity-to-location", _] => parse_map(content, &mut maps[6]),
        _ => panic!("da ist etwas schief gelaufen"),
    }
}

fn parse_seeds(content: &str, seeds: &mut Vec<Range<usize>>) {
    let iter: Vec<usize> = content
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    *seeds = iter
        .windows(2)
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, a)| a[0]..(a[0] + a[1]))
        .collect();
}

fn parse_map(content: &str, map: &mut HashMap<Range<usize>, Range<usize>>) {
    for line in content.lines() {
        let mut nums = line.split_whitespace();
        let dest_cat = nums.next().unwrap().parse().unwrap();
        let source_cat = nums.next().unwrap().parse().unwrap();
        let ran: usize = nums.next().unwrap().parse().unwrap();

        map.insert(source_cat..(source_cat + ran), dest_cat..(dest_cat + ran));
    }
}

fn translate_seed(seed: usize, maps: &mut Vec<HashMap<Range<usize>, Range<usize>>>) -> usize {
    let mut temp = seed;
    for map in maps.into_iter() {
        let result = map.iter().find(|n| n.0.contains(&temp));
        if result.is_some() {
            let x = result.unwrap();
            temp =
                x.1.clone()
                    .nth(temp - (x.0).clone().next().unwrap())
                    .unwrap();
        }
    }

    temp
}
