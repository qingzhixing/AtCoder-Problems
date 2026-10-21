use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let edges: Vec<(usize, usize)> = (0..m).map(|_| (next(tokens), next(tokens))).collect();

    let mut min_delete = m;
    // 枚举 n 个点的染色
    for state in 0..(1 << n) {
        let mut current_delete = 0usize;
        for (u, v) in &edges {
            let u_color = (state >> u) & 1;
            let v_color = (state >> v) & 1;
            if u_color == v_color {
                current_delete += 1;
            }
        }
        min_delete = min_delete.min(current_delete);
    }

    writeln!(out, "{min_delete}").unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    solution(&mut tokens, &mut out);

    out.flush().unwrap();
}
