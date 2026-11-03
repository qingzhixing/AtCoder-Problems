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

/// idx: 当前要填的位, sum: 先前已填入数字的价值和
fn dfs(path: &mut Vec<usize>, idx: usize, sum: usize, n: usize, k: usize) {
    // 最后一位
    if idx == n - 1 {
        let delta = k - sum;
        // 没有合法方案直接抛弃当前方案
        if delta % n != 0 {
            return;
        }
        path[idx] = delta / n;

        for num in path {
            print!("{} ", num);
        }
        println!("");
        return;
    }

    // 枚举当前能填入的所有数字
    for num in 0..=((k - sum) / (idx + 1)) {
        path[idx] = num;
        dfs(path, idx + 1, sum + num * (idx + 1), n, k);
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);

    let mut path = vec![0usize; n];
    dfs(&mut path, 0, 0, n, k);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
