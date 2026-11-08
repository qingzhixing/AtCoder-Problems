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

    // i 作为第一志愿时获得的票数
    // 0 - indexed
    let mut votes = vec![0usize; n];
    // 把 i 作为第一志愿的人的第二志愿
    // 0 - indexed
    let mut second_choice = vec![vec![]; n];

    for _ in 0..m {
        let first: usize = next(tokens);
        let second: usize = next(tokens);
        // to 0 - indexed
        let first = first - 1;
        let second = second - 1;

        votes[first] += 1;
        second_choice[first].push(second);
    }

    // 统计作为第一志愿的最多票数的两个人
    // id, votes
    let mut c1 = (0usize, 0usize);
    let mut c2 = (0usize, 0usize);
    for (idx, &vote) in votes.iter().enumerate() {
        if vote > c1.1 {
            c2 = c1;
            c1 = (idx, vote);
        } else if vote > c2.1 {
            c2 = (idx, vote);
        }
    }

    // 模拟第 idx 人退选
    for idx in 0..n {
        // 修改票数
        for &second in &second_choice[idx] {
            votes[second] += 1;
        }
        let original_vote = votes[idx];
        votes[idx] = 0;

        // 只比较票数变化的人和初始的最大两人
        let check_idx_list = second_choice[idx]
            .iter()
            .copied()
            .chain(once(c1.0))
            .chain(once(c2.0));

        let max_idx = check_idx_list
            .map(|idx| (votes[idx], Reverse(idx)))
            .max()
            .unwrap()
            .1
            .0;

        // 1 - indexed
        println!("{}", max_idx + 1);

        // 状态还原：撤销之前的转移，供下一次循环使用
        votes[idx] = original_vote;
        for &second in &second_choice[idx] {
            votes[second] -= 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
