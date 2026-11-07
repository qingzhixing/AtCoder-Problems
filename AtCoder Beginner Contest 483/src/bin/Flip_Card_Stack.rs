use std::io::{self, Read};
use std::println;
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
    let q: usize = next(tokens);
    let mut a: Vec<i64> = (1..=n as i64).collect();
    for _ in 0..q {
        let x: usize = next(tokens);
        a[..x].reverse();
        a[..x].iter_mut().for_each(|num| *num *= -1);
    }
    for num in a {
        println!("{num}");
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
