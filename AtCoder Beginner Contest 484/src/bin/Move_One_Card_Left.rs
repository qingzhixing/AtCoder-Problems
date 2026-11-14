use std::collections::BTreeMap;
use std::io::{self, Read};
use std::ops::AddAssign;
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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let mut result = 1usize;
    let mut freq = BTreeMap::<usize, usize>::new();
    let mut prev = None::<usize>;

    for (idx, num) in a.into_iter().enumerate() {
        if prev != Some(num) {
            result += idx - *freq.entry(num).or_default();
            prev = Some(num);
        }
        freq.entry(num).or_insert(0).add_assign(1);
    }

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
