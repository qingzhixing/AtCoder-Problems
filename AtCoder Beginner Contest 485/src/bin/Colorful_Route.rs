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
    to: usize,
    color: usize,
}

#[derive(Clone, Copy)]
struct State {
    current: usize,
    prev_color: usize,
    step: usize,
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        let c: usize = next(tokens);

        graph[u].push(Edge { to: v, color: c });
    }

    // 有效访问次数
    let mut visited = vec![0usize; n + 1];
    let mut first_color = vec![0usize; n + 1];

    let mut q = VecDeque::<State>::new();

    // init
    visited[1] = 1;
    q.push_back(State {
        current: 1,
        prev_color: 0,
        step: 0,
    });

    while let Some(State {
        current,
        prev_color,
        step,
    }) = q.pop_front()
    {
        for &Edge { to, color } in &graph[current] {
            if color == prev_color {
                continue;
            }
            if to == n {
                println!("{}", step + 1);
                return;
            }
            if visited[to] == 0 {
                visited[to] += 1;
                first_color[to] = color;
                q.push_back(State {
                    current: to,
                    prev_color: color,
                    step: step + 1,
                });
            } else if visited[to] == 1 && first_color[to] != color {
                visited[to] += 1;
                q.push_back(State {
                    current: to,
                    prev_color: color,
                    step: step + 1,
                });
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
