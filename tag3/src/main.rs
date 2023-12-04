use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy)]
struct Number {
    line: usize,
    begin_col: usize,
    end_col: usize,
}

fn main() {
    let inp: Vec<String> = parse_input().lines().map(|s| s.to_string()).collect();
    let max_line = inp.len() - 1;
    let max_col = inp[0].chars().collect::<Vec<char>>().len() - 1;

    //part 1
    println!(
        "{:?}",
        find_numbers(inp.clone())
            .iter()
            .filter(|n| {
                has_neighbors(
                    **n,
                    max_line,
                    max_col,
                    inp.iter().map(|l| l.chars().collect()).collect(),
                )
            })
            .map(|n| number_to_int(*n, inp.iter().map(|l| l.chars().collect()).collect()))
            .sum::<usize>()
    );

    //part 2

    let mut map = HashMap::new();

    find_numbers(inp.clone()).iter().for_each(|n| {
        has_neighboring_star(
            *n,
            max_line,
            max_col,
            inp.iter().map(|l| l.chars().collect()).collect(),
            &mut map,
        )
    });

    println!(
        "{:?}",
        map.iter()
            .filter(|nums| nums.1.len() == 2)
            .map(|(_, nums)| {
                nums.iter()
                    .map(|n| number_to_int(*n, inp.iter().map(|l| l.chars().collect()).collect()))
                    .product::<usize>()
            })
            .sum::<usize>()
    );
}

fn parse_input() -> String {
    fs::read_to_string("input").unwrap()
}

fn find_numbers(input: Vec<String>) -> Vec<Number> {
    input
        .iter()
        .enumerate()
        .map(|(i, l)| {
            (
                i,
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_digit())
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>(),
            )
        })
        .flat_map(|(i, n)| find_consecutive_numbers(n, i))
        .collect()
}

fn find_consecutive_numbers(input: Vec<usize>, line: usize) -> Vec<Number> {
    let mut ret = Vec::new();
    //let mut num = None;
    let mut begin = None;
    let mut end = None;

    for i in input.iter().enumerate() {
        if i.0 > 0 {
            if input[i.0 - 1] == i.1 - 1 {
                end = Some(end.unwrap() + 1);
            } else {
                ret.push(Number {
                    line,
                    begin_col: begin.unwrap(),
                    end_col: end.unwrap(),
                });
                begin = Some(*i.1);
                end = Some(*i.1);
            }
        } else {
            begin = Some(*i.1);
            end = Some(*i.1);
        }
    }

    if let Some(end) = end {
        ret.push(Number {
            line,
            begin_col: begin.unwrap(),
            end_col: end,
        });
    }
    ret
}

fn has_neighbors(num: Number, max_line: usize, max_col: usize, input: Vec<Vec<char>>) -> bool {
    let y_begin = if num.line > 0 { num.line - 1 } else { num.line };
    let y_end = if num.line == max_line {
        num.line
    } else {
        num.line + 1
    };
    let x_begin = if num.begin_col > 0 {
        num.begin_col - 1
    } else {
        num.begin_col
    };
    let x_end = if num.end_col == max_col {
        num.end_col
    } else {
        num.end_col + 1
    };

    for l in y_begin..(y_end + 1) {
        for c in x_begin..(x_end + 1) {
            if !input[l][c].is_ascii_digit() && input[l][c] != '.' {
                return true;
            }
        }
    }

    false
}

fn has_neighboring_star(
    num: Number,
    max_line: usize,
    max_col: usize,
    input: Vec<Vec<char>>,
    map: &mut HashMap<(usize, usize), Vec<Number>>,
) {
    let y_begin = if num.line > 0 { num.line - 1 } else { num.line };
    let y_end = if num.line == max_line {
        num.line
    } else {
        num.line + 1
    };
    let x_begin = if num.begin_col > 0 {
        num.begin_col - 1
    } else {
        num.begin_col
    };
    let x_end = if num.end_col == max_col {
        num.end_col
    } else {
        num.end_col + 1
    };

    for l in y_begin..(y_end + 1) {
        for c in x_begin..(x_end + 1) {
            if !input[l][c].is_ascii_digit() && input[l][c] == '*' {
                if map.contains_key(&(l, c)) {
                    let mut new_vec = map.get(&(l, c)).unwrap().clone();
                    new_vec.push(num);
                    map.insert((l, c), new_vec);
                } else {
                    map.insert((l, c), vec![num]);
                }
            }
        }
    }
}

fn number_to_int(num: Number, inp: Vec<Vec<char>>) -> usize {
    let num: String = inp[num.line][num.begin_col..num.end_col + 1]
        .iter()
        .collect();

    num.parse().unwrap()
}
