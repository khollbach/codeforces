use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let s = lines.next().unwrap().unwrap();

        let ans = soln(&s);
        if ans {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn soln(s: &str) -> bool {
    s.len() >= 3
        && &s[..2] == "10"
        && s[2..].parse::<u32>().unwrap() >= 2
        && !s[2..].starts_with('0')
}
