use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l) = lines.next() {
        let n: usize = l.unwrap().parse().unwrap();
        assert_ne!(n, 0);
        let s = parse_bitstring(&lines.next().unwrap().unwrap());
        assert_eq!(s.len(), n);
        let t = parse_bitstring(&lines.next().unwrap().unwrap());
        assert_eq!(t.len(), n);

        let ans = soln(&s, &t);
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn parse_bitstring(s: &str) -> Vec<bool> {
    s.chars().map(|c| match c {
        '0' => false,
        '1' => true,
        _ => panic!("not a 0 or 1: {c:?}"),
    }).collect()
}

fn soln(s: &[bool], t: &[bool]) -> bool {
    assert_eq!(s.len(), t.len());
    let n = s.len();

    let first_one = s.iter().copied().position(|b| b == true).unwrap_or(n);

    t[..first_one].iter().all(|&b| b == false)
}
