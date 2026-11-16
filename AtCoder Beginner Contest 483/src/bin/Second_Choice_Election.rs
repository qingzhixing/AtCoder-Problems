use std::cmp::Reverse;
use std::io::{self, Read};
use std::iter::once;
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
    // 第一志愿获票
    let mut votes = vec![0usize; n];
    // 第一志愿为 i 的人的票会给 j
    let mut gives_to = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = next(tokens);
        let b: usize = next(tokens);
        let a = a - 1;
        let b = b - 1;
        votes[a] += 1;
        gives_to[a].push(b);
    }

    // 找到第一志愿最大的两个人
    // idx, vote
    let mut best1 = (0usize, 0usize);
    let mut best2 = (0usize, 0usize);
    for (idx, &vote) in votes.iter().enumerate() {
        if vote > best1.1 {
            best2 = best1;
            best1 = (idx, vote);
        } else if vote > best2.1 {
            best2 = (idx, vote);
        }
    }

    // 模拟第 i 人推选
    for i in 0..n {
        // 修改票数
        for &to in &gives_to[i] {
            votes[to] += 1;
        }
        let original_votes_i = votes[i];
        votes[i] = 0;

        // 参与排序的只有第二志愿以及原来的第一二大
        let check_list = gives_to[i]
            .iter()
            .copied()
            .chain(once(best1.0))
            .chain(once(best2.0));

        let result = check_list
            .map(|idx| (votes[idx], Reverse(idx)))
            .max()
            .unwrap()
            .1
            .0
            + 1;

        println!("{result}");

        // 还原现场
        votes[i] = original_votes_i;
        for &to in &gives_to[i] {
            votes[to] -= 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
