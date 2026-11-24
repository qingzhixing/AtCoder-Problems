use std::collections::VecDeque;
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
struct Edge {
    /// 终点
    to: usize,
    /// 当前边颜色
    color: usize,
}

#[derive(Clone, Copy)]
struct State {
    /// 当前处于的点
    id: usize,
    /// 到这个点的上一条边是什么颜色
    prev_color: usize,
    /// 当前花费步数
    step: usize,
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);

    // 邻接表存图
    let mut graph = vec![vec![]; n + 1];
    for _ in 0..m {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        let c: usize = next(tokens);
        graph[u].push(Edge { to: v, color: c });
    }

    // BFS
    // 当前点合法到达次数
    let mut visited = vec![0usize; n + 1];
    // 第一次到当前点的颜色
    let mut first_color = vec![0usize; n + 1];

    let mut q = VecDeque::<State>::new();

    // init state
    visited[1] += 1;
    q.push_back(State {
        id: 1,
        prev_color: 0,
        step: 0,
    });

    while let Some(State {
        id,
        prev_color,
        step,
    }) = q.pop_front()
    {
        for &Edge {
            to,
            color: nxt_color,
        } in &graph[id]
        {
            // 不能连续经过两个相同颜色的边
            if prev_color == nxt_color {
                continue;
            }

            // 下一个点是终点，可以剪枝输出
            if to == n {
                println!("{}", step + 1);
                return;
            }

            // 第一次到则插入
            if visited[to] == 0 {
                first_color[to] = nxt_color;
                visited[to] += 1;
                q.push_back(State {
                    id: to,
                    prev_color: nxt_color,
                    step: step + 1,
                });
                continue;
            }

            // 第二次异色插入
            if visited[to] == 1 && nxt_color != first_color[to] {
                visited[to] += 1;
                q.push_back(State {
                    id: to,
                    prev_color: nxt_color,
                    step: step + 1,
                });
                continue;
            }
        }
    }

    println!("-1");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
