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

#[derive(Clone, Copy)]
struct Zone {
    sum: i64,
    start: i64,
    end: i64,
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    // a 前缀和
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    // 统计前后缀最大最小前缀和
    // 前缀 记录 0 到 i - 1 步，不包括 i
    // 1 - indexed, 0 ~ i - 1 映射到 1 ~ i
    // 访问 pre_xxx[i] 就自然不包括 i 了
    let mut pre_max = vec![0i64; n + 1];
    let mut pre_min = vec![0i64; n + 1];
    pre_max[0] = s[0];
    pre_min[0] = s[0];
    for i in 1..=n {
        pre_max[i] = pre_max[i - 1].max(s[i]);
        pre_min[i] = pre_min[i - 1].min(s[i]);
    }
    // 后缀 记录 i 到 N 步，包括 i
    // 0 - indexed
    let mut suf_max = vec![0i64; n];
    let mut suf_min = vec![0i64; n];
    suf_max[n - 1] = s[n];
    suf_min[n - 1] = s[n];
    for i in (0..n - 1).rev() {
        suf_max[i] = suf_max[i + 1].max(s[i + 1]);
        suf_min[i] = suf_min[i + 1].min(s[i + 1]);
    }

    // 不翻转任何元素
    let mut result = pre_max[n] - pre_min[n] + 1;

    // 模拟翻转元素 i
    for i in 0..n {
        let current_max = pre_max[i].max(suf_max[i] - 2 * a[i]);
        let current_min = pre_min[i].min(suf_min[i] - 2 * a[i]);
        result = result.max(current_max - current_min + 1);
    }

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
