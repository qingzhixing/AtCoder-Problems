use std::io::{self, Read};
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
    let s = next_str(tokens);

    // 左切分点左界
    let mut left = 0usize;
    let mut left_contains = vec![false; 3];
    for i in 0..n {
        left_contains[s[i] as usize - 'A' as usize] = true;
        if left_contains.iter().all(|&x| x == true) {
            left = i;
            break;
        }
    }
    drop(left_contains);
    let left = left;

    // 右切分点右界
    let mut right = 0usize;
    let mut right_contains = vec![false; 3];
    for i in (0..n).rev() {
        right_contains[s[i] as usize - 'A' as usize] = true;
        if right_contains.iter().all(|&x| x == true) {
            right = i;
            break;
        }
    }
    drop(right_contains);
    let right = right;

    // 判断有解性
    if left + 1 >= right {
        println!("0");
        return;
    }

    let mut mid_freq = vec![0usize; 3];
    for i in left + 1..right {
        mid_freq[s[i] as usize - 'A' as usize] += 1;
    }
    if mid_freq.iter().any(|&x| x == 0) {
        println!("0");
        return;
    }
    let mid_freq = mid_freq;

    // [left, right] 区间大小，用于枚举左右界
    let len = right - left + 1;

    // 区间前缀频率
    let mut mid_pre_freq = vec![vec![0usize; 3]; len];
    for i in 1..len {
        mid_pre_freq[i] = mid_pre_freq[i - 1].clone();
        mid_pre_freq[i][s[left + i] as usize - 'A' as usize] += 1;
    }
    let mid_pre_freq = mid_pre_freq;

    // 区间后缀频率
    let mut mid_suf_freq = vec![vec![0usize; 3]; len];
    for i in (0..len - 1).rev() {
        mid_suf_freq[i] = mid_suf_freq[i + 1].clone();
        mid_suf_freq[i][s[left + i] as usize - 'A' as usize] += 1;
    }
    let mid_suf_freq = mid_suf_freq;

    let mut result = 0usize;
    // 从左往右枚举左分割点
    'outer: for i in 0..len {
        let mut mid_rest: Vec<_> = vec![0usize; 3];
        for alpha in 0..3 {
            if mid_freq[alpha] <= mid_pre_freq[i][alpha] {
                break 'outer;
            } else {
                mid_rest[alpha] = mid_freq[alpha] - mid_pre_freq[i][alpha];
            }
        }

        // 搜索最小右分割点
        let min_right = mid_suf_freq.partition_point(|suf_freq| {
            (0..3)
                .map(|alpha| suf_freq[alpha] >= mid_rest[alpha])
                .any(|x| x == true)
        });

        result += len - min_right;
    }

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
