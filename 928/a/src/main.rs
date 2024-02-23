use std::io;

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(l) = lines.next() {
        let ans = soln(&l);
        println!("{ans}");
    }
}

fn soln(s: &str) -> char {
    let num_a = s.chars().filter(|&c| c == 'A').count();
    let num_b = s.chars().filter(|&c| c == 'B').count();
    if num_a > num_b {
        'A'
    } else {
        'B'
    }
}
