use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 510;
const _MAX_M: usize = 1e9 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

/// number === remainder (mod modular) 所需要的最小代价
fn dist(number: usize, remainder: usize, modular: usize) -> usize {
    (remainder + modular - number % modular) % modular
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let l: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    const INF: usize = 1e18 as usize;
    // dp[r] = 已经处理前若干组后，当前第一窗口 余数和 === r (mod M) 的最小代价
    let mut dp = vec![INF; m];
    // a[0] = 0;
    dp[0] = 0;

    // 枚举每一组
    for group_idx in 0..l {
        // cost[c] 将当前组所有元素变为 === c (mod M) 的代价
        let mut cost = vec![0; m];

        // 枚举余数
        for remainder in 0..m {
            for number_idx in (group_idx..n).step_by(l) {
                cost[remainder] += dist(a[number_idx], remainder, m);
            }
        }

        // 转移 dp
        let mut next_dp = vec![INF; m];
        for previous_remainder in 0..m {
            if dp[previous_remainder] == INF {
                continue;
            }
            for current_remainder in 0..m {
                let next_remainder = (previous_remainder + current_remainder) % m;
                next_dp[next_remainder] =
                    next_dp[next_remainder].min(dp[previous_remainder] + cost[current_remainder]);
            }
        }
        dp = next_dp;
    }

    writeln!(out, "{}", dp[0]).unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    solution(&mut tokens, &mut out);

    out.flush().unwrap();
}
