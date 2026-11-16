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

struct Fenwick {
    n: usize,
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0; n + 1],
        }
    }

    /// 0-indexed 单点更新：在位置 idx 处增加 delta
    #[inline]
    fn add(&mut self, idx: usize, delta: i64) {
        let mut i = idx + 1;
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg(); // 等价于 lowbit(i)
        }
    }

    /// 查询 0-indexed 左闭右开区间 [0, pos) 的前缀和（即前 pos 个桶的总和）
    #[inline]
    fn sum_prefix(&self, pos: usize) -> i64 {
        let mut sum = 0;
        let mut i = pos.min(self.n);
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let l: i64 = next(tokens);
    let r: i64 = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 1 - indexed prefix sum
    let mut s: Vec<i64> = vec![0i64; n + 1];
    for i in 1..=n {
        s[i] = s[i - 1] + a[i - 1];
    }
    let s = s;

    // 将 s 离散化做为树状数组下标使用
    let mut coords = s.clone();
    coords.sort_unstable();
    coords.dedup();

    // 记录 s[i] 映射到的 coords 下标
    let ranks: Vec<_> = s.iter().map(|x| coords.binary_search(x).unwrap()).collect();

    // 记录 元素和 >= x 的子数组个数
    let count_at_least = |x: i64| -> usize {
        let mut result = 0usize;
        let mut fenwick = Fenwick::new(coords.len());

        for (i, &si) in s.iter().enumerate() {
            let upper_bound = si - x;
            // 找到第一个 > si - x 的点，该点前面的都符合要求
            let first_invalid = coords.partition_point(|&x| x <= upper_bound);
            // 累加答案
            result += fenwick.sum_prefix(first_invalid) as usize;
            // 该点加入统计
            fenwick.add(ranks[i], 1);
        }

        result
    };

    let result = count_at_least(l) - count_at_least(r + 1);
    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
