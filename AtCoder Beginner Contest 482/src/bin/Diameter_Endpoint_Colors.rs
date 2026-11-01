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
    let mut n: usize = next(tokens);

    // 将每条边插入一个新的节点，这样我们之后求的树的直径一定为偶数
    // 0 - indexed
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n * 2 - 1];
    for i in 0..n - 1 {
        let mut u: usize = next(tokens);
        let mut v: usize = next(tokens);

        // 0 - indexed
        u -= 1;
        v -= 1;

        // u v 之间插入一个 w
        let w = n + i;
        adj[u].push(w);
        adj[v].push(w);
        adj[w].push(u);
        adj[w].push(v);
    }
    let adj = adj;

    // 所有点的颜色
    // 虚拟节点颜色默认为 0, 没影响，因为它不可能为叶子节点。
    let mut colors = vec![0usize; n * 2 - 1];
    for i in 0..n {
        let mut color: usize = next(tokens);

        // color 从 0 开始, 方便作为下标索引
        color -= 1;

        colors[i] = color;
    }
    let colors = colors;

    // 将 n 变为插入虚拟节点之后的数量
    let n = n * 2 - 1;

    // 从 root 开始进行 bfs, 求各点到 root 的距离
    let bfs_dist = |root: usize| -> Vec<usize> {
        let mut dist = vec![n; n];
        dist[root] = 0;

        let mut q = VecDeque::from([root]);
        while let Some(current) = q.pop_front() {
            for &nxt in &adj[current] {
                if dist[nxt] > dist[current] + 1 {
                    dist[nxt] = dist[current] + 1;
                    q.push_back(nxt);
                }
            }
        }
        dist
    };

    // dist 当中最远的那个点
    let furthest_id =
        |dist: &Vec<usize>| dist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap().0;

    // 求数的直径
    let dist0 = bfs_dist(0);
    // 直径端点之一
    let v1 = furthest_id(&dist0);
    // 所有点到 v1 的距离
    let dist1 = bfs_dist(v1);
    // 直径端点之一
    let v2 = furthest_id(&dist1);
    // 所有点到 v2 的距离
    let dist2 = bfs_dist(v2);

    // 树的直径
    let diam = dist2[v1];
    // 由于插入虚拟节点，树的直径长度一定为偶数
    assert!(diam % 2 == 0);

    // 寻找树的中心
    let mut center = n + 1;
    for i in 0..n {
        // 到直径端点距离相等
        if dist1[i] == diam / 2 && dist2[i] == diam / 2 {
            center = i;
        }
    }
    let center = center;
    // 每个点到 center 的距离
    let dist = bfs_dist(center);

    drop(dist0);
    drop(dist1);
    drop(dist2);

    // 统计答案
    let mut result = 0usize;
    compile_error!("TODO: Unimplemented.");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
