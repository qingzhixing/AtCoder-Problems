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

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 执行 a0 ~ a(i-1) 到达的左端
    let mut pre_min = vec![0i64; n];
    // 执行 a0 ~ a(i-1) 到达的右端
    let mut pre_max = vec![0i64; n];
    // 回退 a(i+1) ~ a(n-1)  到达的左端
    let mut suf_min = vec![0i64; n];
    // 回退 a(i+1) ~ a(n-1)  到达的右端
    let mut suf_max = vec![0i64; n];

    // 初始化前缀
    let mut current = 0i64;
    for i in 1..n {
        // 执行 a(i-1)
        current += a[i - 1];
        pre_min[i] = pre_min[i - 1].min(current);
        pre_max[i] = pre_max[i - 1].max(current);
    }
    let pre_min = pre_min;
    let pre_max = pre_max;

    // 初始化后缀
    current += a[n - 1]; // current 于 prefix[n]
    for i in (0..n).rev() {
        if i >= n - 1 || suf_min[i + 1] > current {
            suf_min[i] = current;
        } else {
            suf_min[i] = suf_min[i + 1];
        }
        if i >= n - 1 || suf_max[i + 1] < current {
            suf_max[i] = current;
        } else {
            suf_max[i] = suf_max[i + 1];
        }
        // 回退 ai
        current -= a[i];
    }
    let suf_min = suf_min;
    let suf_max = suf_max;

    let total_min = pre_min[0].min(suf_min[0]);
    let total_max = pre_max[0].max(suf_max[0]);

    let mut result = (total_max - total_min + 1) as usize;

    // 枚举每一位反转操作
    for i in 0..n {
        let left = pre_min[i].min(suf_min[i] - 2 * a[i]);
        let right = pre_max[i].max(suf_max[i] - 2 * a[i]);
        result = result.max((right - left).abs() as usize + 1);
    }
    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
