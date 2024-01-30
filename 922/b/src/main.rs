use std::{io, iter::zip};

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for nxy in lines.chunks(3) {
        let xs = nxy[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ys = nxy[2]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (xs, ys) = soln(xs, ys);
        print_list(&xs);
        print_list(&ys);
    }
}

fn print_list(xs: &[u32]) {
    for x in xs {
        print!("{x} ");
    }
    println!();
}

fn soln(xs: Vec<u32>, ys: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let mut pairs: Vec<_> = zip(xs, ys).collect();
    pairs.sort_unstable();
    pairs.into_iter().unzip()
}
