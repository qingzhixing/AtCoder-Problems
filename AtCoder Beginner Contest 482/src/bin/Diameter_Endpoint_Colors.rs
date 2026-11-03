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

    // 在每个点之间插入一个虚拟节点，让直径确定为偶数
    let mut adj = vec![Vec::<usize>::new(); n * 2 - 1];
    for i in 0..n - 1 {
        let u: usize = next(tokens);
        let v: usize = next(tokens);

        // 0 - indexed
        let u = u - 1;
        let v = v - 1;
        let w = n + i;

        // u v 之间插入 w
        adj[u].push(w);
        adj[v].push(w);
        adj[w].push(u);
        adj[w].push(v);
    }

    let mut colors = vec![0usize; n * 2 - 1];
    for i in 0..n {
        let color = next(tokens);
        colors[i] = color;
    }
    let colors = colors;

    // 变为总点数
    let n = n * 2 - 1;

    // 从 root 出发，求各点到 root 的长度
    let bfs_dist = |root: usize| -> Vec<usize> {
        let mut dist = vec![n; n];
        dist[root] = 0;
        let mut q = VecDeque::from([root]);

        while let Some(current) = q.pop_front() {
            let current_dist = dist[current];
            for &nxt in &adj[current] {
                if dist[nxt] > current_dist + 1 {
                    dist[nxt] = current_dist + 1;
                    q.push_back(nxt);
                }
            }
        }

        dist
    };

    let furthest_idx =
        |dist: &Vec<usize>| -> usize { dist.iter().enumerate().max_by_key(|e| e.1).unwrap().0 };

    // BFS 求直径
    let dist0 = bfs_dist(0);
    let v1 = furthest_idx(&dist0);
    let dist1 = bfs_dist(v1);
    let v2 = furthest_idx(&dist1);
    let dist2 = bfs_dist(v2);

    let diam = dist2[v1];
    // 直径一定为偶数
    debug_assert!(diam % 2 == 0);

    // 求树的中心
    let mut center = 0usize;
    for i in 0..n {
        if dist1[i] == diam / 2 && dist2[i] == diam / 2 {
            center = i;
            break;
        }
    }
    let center = center;
    let dist = bfs_dist(center);

    // 从 center 进行 DFS，找到各分支满足条件的点，然后加入统计
    // 当前已遍历的其他分支的颜色频率
    let mut total_color_freq = vec![0usize; n];
    let mut result = 0usize;

    // 枚举各个分支进行 dfs
    for &branch in &adj[center] {
        let mut branch_valid_points: Vec<usize> = vec![];

        // 调用栈 current, parent
        let mut stack = vec![(branch, center)];
        while let Some((current, parent)) = stack.pop() {
            if dist[current] == diam / 2 {
                branch_valid_points.push(current);
            }

            for &nxt in &adj[current] {
                if nxt == parent {
                    continue;
                }
                stack.push((nxt, current));
            }
        }

        // 累计答案
        for &point in &branch_valid_points {
            let color = colors[point];
            result += total_color_freq[color];
        }

        // 加入统计
        for &point in &branch_valid_points {
            let color = colors[point];
            total_color_freq[color] += 1;
        }
    }

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
