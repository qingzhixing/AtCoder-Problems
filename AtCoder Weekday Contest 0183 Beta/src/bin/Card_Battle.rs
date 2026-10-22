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
    let _k: usize = next(tokens);
    let max_a: i64 = (0..n).map(|_| next(tokens)).max().unwrap();
    let result = (0..m)
        .map(|_| next(tokens))
        .position(|number: i64| number > max_a)
        .unwrap_or(m);
    writeln!(out, "{result}").unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    solution(&mut tokens, &mut out);

    out.flush().unwrap();
}
