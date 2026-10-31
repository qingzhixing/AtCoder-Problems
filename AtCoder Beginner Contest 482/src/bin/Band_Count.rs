use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{iter, println};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

struct Block {
    row: usize,
    col: usize,
}

struct Query {
    op: usize,
    start: usize,
    end: usize,
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let _h: usize = next(tokens);
    let _w: usize = next(tokens);
    let n: usize = next(tokens);
    let q: usize = next(tokens);

    let mut blocks = Vec::<Block>::new();
    let mut queries = Vec::<Query>::new();
    let mut positions = Vec::<usize>::new();

    for _ in 0..n {
        let row: usize = next(tokens);
        let col: usize = next(tokens);
        blocks.push(Block { row, col });
        positions.push(row);
        positions.push(col);
    }

    for _ in 0..q {
        let op: usize = next(tokens);
        let start: usize = next(tokens);
        let end: usize = next(tokens);
        queries.push(Query { op, start, end });
        positions.push(start);
        positions.push(end);
    }

    // 离散化
    positions.sort_unstable();
    positions.dedup();

    let size = positions.len();
    let mut row_cnt = vec![0usize; size];
    let mut col_cnt = vec![0usize; size];

    for Block { row, col } in blocks {
        row_cnt[positions.binary_search(&row).unwrap()] += 1;
        col_cnt[positions.binary_search(&col).unwrap()] += 1;
    }

    // 前 i-1 个数前缀和
    let pre_row: Vec<_> = iter::once(0usize)
        .chain(row_cnt.iter().scan(0usize, |prev, value| {
            *prev += value;
            Some(*prev)
        }))
        .collect();

    let pre_col: Vec<_> = iter::once(0usize)
        .chain(col_cnt.iter().scan(0usize, |prev, value| {
            *prev += value;
            Some(*prev)
        }))
        .collect();

    for Query {
        op,
        mut start,
        mut end,
    } in queries
    {
        start = positions.binary_search(&start).unwrap();
        end = positions.binary_search(&end).unwrap();

        match op {
            1 => {
                println!("{}", pre_row[end + 1] - pre_row[start]);
            }
            2 => {
                println!("{}", pre_col[end + 1] - pre_col[start]);
            }
            _ => {
                println!("Invalid Operation!!!");
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
