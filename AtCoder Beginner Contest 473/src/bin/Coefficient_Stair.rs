use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{print, println, vec};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn dfs(path: &mut Vec<usize>, idx: usize, sum: usize, n: usize, k: usize) {
    if idx > n {
        // 输出 path
        for num in path {
            print!("{num} ");
        }
        println!("");
        return;
    }

    // 正在填充第 n 位
    if idx == n {
        // 此时无解
        if (k - sum) % n != 0 {
            return;
        }
        path[idx - 1] = (k - sum) / n;
        dfs(path, idx + 1, k, n, k);
        return;
    }

    // 枚举当前位置能选的所有数字
    for num in 0..=(k - sum) / idx {
        path[idx - 1] = num;
        dfs(path, idx + 1, sum + idx * num, n, k);
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);

    let mut path = vec![0usize; n];
    dfs(&mut path, 1, 0, n, k);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
