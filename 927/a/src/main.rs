use std::io;

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(_n) = lines.next() {
        let s = lines.next().unwrap();
        let ans = soln(&s);
        println!("{}", ans);
    }
}

fn soln(s: &str) -> usize {
    let n = s.len();
    let end = s.find("**").unwrap_or(n);
    let prefix = &s[..end];
    let num_coins = prefix.chars().filter(|&c| c == '@').count();
    num_coins
}
