use std::cmp::{max, min};
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
    let n: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    for i in 0..n {
        // 传递到左边第一个
        let left_first = (if i == 0 { 0 } else { a[i] - a[i - 1] }) / 2;
        // 左边第一个传到最左边
        let left_end = (if i == 0 { 0 } else { a[i - 1] - a[0] }) / 2;
        // 传递到右边第一个
        let right_first = (if i == n - 1 { 0 } else { a[i + 1] - a[i] }) / 2;
        // 右边第一个传到最右边
        let right_end = (if i == n - 1 { 0 } else { a[n - 1] - a[i + 1] }) / 2;

        // 先走左边;
        let left_result = left_first + max(right_first + right_end, left_end);
        // 先走右边
        let right_result = right_first + max(left_first + left_end, right_end);
        let result = min(left_result, right_result);
        println!("{result}");
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
