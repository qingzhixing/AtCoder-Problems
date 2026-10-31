use std::io::{self, Read};
use std::println;
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let map: Vec<Vec<char>> = (0..5).map(|_| next_str(tokens)).collect();
    let mut result = 6usize;

    // 垂直
    for row in 0..5 {
        let mut needed = 0usize;
        for col in 0..5 {
            if map[row][col] == '.' {
                needed += 1;
            }
        }
        result = result.min(needed);
    }

    // 水平
    for col in 0..5 {
        let mut needed = 0usize;
        for row in 0..5 {
            if map[row][col] == '.' {
                needed += 1;
            }
        }
        result = result.min(needed);
    }

    // 对角线
    let mut forward = 0usize;
    let mut backward = 0usize;
    for i in 0..5 {
        if map[i][i] == '.' {
            forward += 1;
        }

        if map[i][5 - i - 1] == '.' {
            backward += 1;
        }
    }
    result = result.min(forward);
    result = result.min(backward);

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
