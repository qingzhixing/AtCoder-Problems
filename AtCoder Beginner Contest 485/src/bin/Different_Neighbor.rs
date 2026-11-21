use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{print, vec};

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
    let c: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 找前方
    let mut front_dist = vec![1e9 as usize; n];
    for i in 1..n {
        if c[i - 1] == c[i] {
            front_dist[i] = front_dist[i].min(front_dist[i - 1] + 1);
        } else {
            front_dist[i] = 1;
        }
    }
    // 找后方
    let mut back_dist = vec![1e9 as usize; n];
    for i in (0..n - 1).rev() {
        if c[i + 1] == c[i] {
            back_dist[i] = back_dist[i].min(back_dist[i + 1] + 1);
        } else {
            back_dist[i] = 1;
        }
    }

    for i in 0..n {
        print!("{} ", front_dist[i].min(back_dist[i]));
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
