use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const INF: usize = 1e9 as usize;

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

/// from 只增加的情况下变为与 to 同余的代价
fn mod_dist(from: usize, to: usize, modular: usize) -> usize {
    ((to % modular) + modular - (from % modular)) % modular
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let l: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 将 a 分为 L 组，按下标 mod L 余数分组
    let mut groups = vec![vec![]; l];
    for (idx, num) in a.into_iter().enumerate() {
        groups[idx % l].push(num % m);
    }

    // dp[i] 表示处理前若干组后 a[1] ~ a[l] 区间和余数变为 i 的最小代价
    let mut dp = vec![INF; m];
    // 处理前 0 组时，余数一定为 0
    dp[0] = 0;

    // 枚举每一组
    for group in groups {
        // 统一当前组余数为 c 的代价
        let mut cost = vec![0usize; m];
        for r in 0..m {
            for &num in &group {
                cost[r] += mod_dist(num, r, m);
            }
        }
        let cost = cost;

        // 下一阶段状态
        let mut next_dp = vec![INF; m];
        // 状态转移，前一个余数转变为后一个余数的代价
        for previous_remainder in 0..m {
            // 前一个状态不存在直接跳过
            if dp[previous_remainder] == INF {
                continue;
            }
            // 枚举当前目标余数，进行转移
            for r in 0..m {
                let nxt_remainder = (previous_remainder + r) % m;
                next_dp[nxt_remainder] =
                    next_dp[nxt_remainder].min(dp[previous_remainder] + cost[r]);
            }
        }

        // 写回 dp
        dp = next_dp;
    }

    // 最终状态 a[1] ~ a[l] 区间和应为 0
    println!("{}", dp[0]);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
