use std::{fs, cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn main() {
    let mut input = parse_input(read_input());
    input.sort_by(|a, b| {
        compare_hands(&a.0, &b.0)
    });

    println!("{}", input.iter().enumerate().map(|(i, (_, v))| (i + 1) * v).sum::<usize>());
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: String) -> Vec<(Vec<usize>, usize)> {
    input.lines().map(|l| {
        let mut line = l.split_whitespace();

        (line.next().unwrap().chars().map(|c| {
            if c.is_ascii_digit() {
                c.to_string().parse::<usize>().unwrap()
            } else {
                match c {
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Das kann überhaupt nicht sein")
                }
            }
        }).collect(), line.next().unwrap().parse().unwrap())
    }).collect()
}

fn compare_hands(a: &Vec<usize>, b: &Vec<usize>) -> Ordering {
    let type_of_a = hand_type(a);
    let type_of_b = hand_type(b);

    if type_of_a.cmp(&type_of_b) == Ordering::Equal {
        return a.cmp(&b)
    }

    type_of_a.cmp(&type_of_b)
}

fn hand_type(a: &Vec<usize>) -> HandType {
    let mut map: HashMap<&usize, Vec<usize>> = HashMap::new();

    for (i, num) in a.iter().enumerate() {
        if map.contains_key(num) {
            let mut vec = map.get(num).unwrap().clone();
            vec.push(i);
            map.insert(num, vec);

        } else {
            map.insert(num, vec![i]);
        }
    }

    let set = map.iter().map(|e| e.1.len()).collect::<Vec<usize>>();

    if set.contains(&5) {
        return HandType::Five
    } 
    if set.contains(&4) {
        if map.get(&1).is_some() && (map.get(&1).unwrap().len() == 1 || map.get(&1).unwrap().len() == 4) {
            return HandType::Five;
        }
        return HandType::Four
    } 
    if set.contains(&3) && set.contains(&2) {
        if map.get(&1).is_some() && (map.get(&1).unwrap().len() == 2 || map.get(&1).unwrap().len() == 3) {
            return HandType::Five;
        }
        return HandType::FullHouse
    } 
    if set.contains(&3) {
        if map.get(&1).is_some() && (map.get(&1).unwrap().len() == 1 || map.get(&1).unwrap().len() == 3) {
            return HandType::Four;
        }
        return HandType::Three
    } 
    if set.iter().filter(|e| **e == 2).count() == 2 {
        if map.get(&1).is_some() && map.get(&1).unwrap().len() == 1 {
            return HandType::FullHouse;
        }
        if map.get(&1).is_some() && map.get(&1).unwrap().len() == 2 {
            return HandType::Four;
        }
        return HandType::TwoPair;
    }
    if set.contains(&2) {
        if map.get(&1).is_some() && (map.get(&1).unwrap().len() == 1 || map.get(&1).unwrap().len() == 2){
            return HandType::Three;
        }
        return HandType::OnePair;
    }

    if map.get(&1).is_some() && map.get(&1).unwrap().len() == 1{
        return HandType::OnePair;
    }
    HandType::HighCard
}