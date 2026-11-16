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

const INF: usize = 1e9 as usize;

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let l: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    // 按 下标 % l 余数分组
    let mut groups: Vec<Vec<_>> = vec![vec![]; l];
    for (i, x) in a.into_iter().enumerate() {
        groups[i % l].push(x % m);
    }

    // from 只增加的情况下变为和 to 同余的最小代价
    let dist = |from: usize, to: usize| -> usize { ((to % m) + m - (from % m)) % m };

    // dp[i] = 在处理完若干组后，前 L 个数变为与 i 同余的最小代价
    let mut dp = vec![INF; m];
    // 一组都不处理时，代价为 0
    dp[0] = 0;
    for group in groups {
        // 枚举当前组的余数变为 x 时需要的代价
        let cost: Vec<usize> = (0..m)
            .map(|x| group.iter().map(|&num| dist(num, x)).sum())
            .collect();

        // 状态转移
        let mut nxt_dp = vec![INF; m];
        for pre_mod in 0..m {
            // 前一个状态不存在
            if dp[pre_mod] == INF {
                continue;
            }
            for group_mod in 0..m {
                let nxt_mod = (pre_mod + group_mod) % m;
                nxt_dp[nxt_mod] = nxt_dp[nxt_mod].min(dp[pre_mod] + cost[group_mod]);
            }
        }

        dp = nxt_dp;
    }

    println!("{}", dp[0]);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
