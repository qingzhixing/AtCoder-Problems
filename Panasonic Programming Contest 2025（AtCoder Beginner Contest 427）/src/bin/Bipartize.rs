use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

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
    let mut edges: Vec<(usize, usize)> = vec![];
    for _ in 0..m {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        // 0 - indexed
        let u = u - 1;
        let v = v - 1;

        edges.push((u, v));
    }

    let mut result = 1e9 as usize;
    for state in 0..(1 << n) {
        // 当前方案代价
        let mut cost = 0usize;
        for &(u, v) in &edges {
            if ((state >> u) & 1) == ((state >> v) & 1) {
                cost += 1;
            }
        }
        result = result.min(cost);
    }

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
