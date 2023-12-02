use std::fs;

fn main() {
    println!(
        "{}",
        parse_input()
            .lines()
            .filter_map(|l| is_possible(l.to_string(), 12, 13, 14))
            .sum::<usize>()
    );
}

fn parse_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn _calc_power(input: String) -> usize {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    let line: Vec<&str> = input.split(':').collect();

    line.get(1).unwrap().split([',', ';']).for_each(|c| {
        match c.trim().split(' ').collect::<Vec<&str>>()[..] {
            [x, "blue"] => {
                if x.parse::<usize>().unwrap() > max_blue {
                    max_blue = x.parse().unwrap()
                }
            }
            [x, "red"] => {
                if x.parse::<usize>().unwrap() > max_red {
                    max_red = x.parse().unwrap()
                }
            }
            [x, "green"] => {
                if x.parse::<usize>().unwrap() > max_green {
                    max_green = x.parse().unwrap()
                }
            }
            _ => panic!(),
        }
    });
    max_red * max_blue * max_green
}

fn is_possible(input: String, red: usize, green: usize, blue: usize) -> Option<usize> {
    let line: Vec<&str> = input.split(':').collect();

    let number_game: usize = line
        .first()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let x = line
        .get(1)
        .unwrap()
        .split([',', ';'])
        .map(|c| match c.trim().split(' ').collect::<Vec<&str>>()[..] {
            [x, "blue"] => {
                if x.parse::<usize>().unwrap() <= blue {
                    Some(())
                } else {
                    None
                }
            }
            [x, "red"] => {
                if x.parse::<usize>().unwrap() <= red {
                    Some(())
                } else {
                    None
                }
            }
            [x, "green"] => {
                if x.parse::<usize>().unwrap() <= green {
                    Some(())
                } else {
                    None
                }
            }
            _ => panic!(),
        })
        .collect::<Option<()>>();

    if x.is_some() {
        Some(number_game)
    } else {
        None
    }
}
