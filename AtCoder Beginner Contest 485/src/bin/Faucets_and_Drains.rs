use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{println, vec};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let s: Vec<Vec<char>> = (0..n).map(|_| next_str(tokens)).collect();

    let mut row_block = vec![true; n];
    let mut col_block = vec![true; m];
    for row in 0..n {
        for col in 0..m {
            if s[row][col] == 'o' {
                row_block[row] = false;
                col_block[col] = false;
            }
        }
    }

    for row in 0..n {
        if row_block[row] {
            println!("Faucet {}", row + 1);
            return;
        }
    }
    for col in 0..m {
        if col_block[col] {
            println!("Drain {}", col + 1);
            return;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
