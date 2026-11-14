use std::io::{self, Read};
use std::println;
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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let b: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    // 前缀相等的元素个数
    let mut pre_eq = 0usize;
    for i in 0..n {
        if a[i] != b[i] {
            break;
        }
        pre_eq += 1;
    }
    let pre_eq = pre_eq;
    // 后缀相等的元素个数
    let mut suf_eq = 0usize;
    for i in (0..n).rev() {
        if a[i] != b[i] {
            break;
        }
        suf_eq += 1;
    }
    let suf_eq = suf_eq;

    // 已经相等
    if pre_eq + suf_eq >= n {
        println!("Yes");
        return;
    }

    // 仅一个不相等
    if n - pre_eq - suf_eq == 1 {
        println!("No");
        return;
    }

    // 不相等区域
    let start = pre_eq;
    let end = n - 1 - suf_eq;
    let unequal_size = end - start + 1;
    // 判断是不是区域后面变到前面
    let mut from_back = a[end] == b[start];
    if from_back {
        for i in 0..unequal_size - 1 {
            if a[start + i] != b[start + 1 + i] {
                from_back = false;
                break;
            }
        }
    }

    // 判断是不是区域前面变到后面
    let mut from_front = a[start] == b[end];
    if from_front {
        for i in 0..unequal_size - 1 {
            if a[start + 1 + i] != b[start + i] {
                from_front = false;
                break;
            }
        }
    }

    if !from_back && !from_front {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
