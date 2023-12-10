use std::fs;

fn main() {
    println!("{}", parse_input(read_input()).iter().map(|v| extrapolate(v)).sum::<i64>());
}

fn read_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn parse_input(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(line: &Vec<i64>) -> i64{
    let mut vec_lines = vec![line.clone()];
    

    while vec_lines[vec_lines.len() - 1].iter().any(|x| *x != 0) {
        vec_lines.push(vec_lines[vec_lines.len() - 1].windows(2).map(|w| w[1] - w[0]).collect())
    }

    let mut extrapolation = 0;

    vec_lines.iter().rev().skip(1).for_each(|r| extrapolation = r[0] - extrapolation);
    
    extrapolation
}