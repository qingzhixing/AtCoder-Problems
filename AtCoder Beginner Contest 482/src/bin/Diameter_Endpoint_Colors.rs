use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::vec;

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
    // 1 - indexed
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut color: Vec<usize> = vec![0usize; n + 1];

    for _ in 0..n - 1 {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        adj[u].push(v);
        adj[v].push(u);
    }

    for i in 0..n {
        let c: usize = next(tokens);
        color[i + 1] = c;
    }

    // 用于找直径的起始点
    let mut start = 1usize;

    // 到根的距离
    let mut dist: Vec<Option<usize>> = vec![None; n + 1];
    dist[1] = Some(0);

    // 当前点是否为叶子节点
    let mut leaf = vec![false; n + 1];

    // BFS
    // 初始化每一点到树根的距离
    let mut q = VecDeque::from([1usize]);
    while let Some(current) = q.pop_front() {
        let current_dist = dist[current].unwrap();

        if adj[current].len() == 1 {
            leaf[current] = true;
        }

        for &nxt in &adj[current] {
            // 已经访问过则跳过
            if let Some(_) = dist[nxt] {
                continue;
            }
            dist[nxt] = Some(current_dist + 1);
            q.push_back(nxt);

            // 更新 start
            let start_dist = dist[start].unwrap();
            if current_dist + 1 > start_dist {
                start = nxt;
            }
        }
    }

    // 求直径
    // let mut end =
    compile_error!("TODO: Unimplemented.");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
