use std::io;
use std::cmp::min;

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for l in lines {
        let n: u8 = l.parse().unwrap();
        let ans = soln(n);
        println!("{}", display(ans));
    }
}

fn display(offsets: [u8; 3]) -> String {
    offsets
        .into_iter()
        .map(|idx| (b'a' + idx) as char)
        .collect()
}

fn soln(n: u8) -> [u8; 3] {
    let n = n - 3; // 0-based indexing for the alphabet, please

    let c = min(n, 25);
    let n = n - c;

    let b = min(n, 25);
    let n = n - b;

    let a = n;

    [a, b, c]
}
