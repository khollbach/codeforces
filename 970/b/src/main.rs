use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let s = lines.next().unwrap().unwrap();
        assert_eq!(s.len(), n);

        if can_square(&s) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn can_square(s: &str) -> bool {
    if s == "1111" {
        return true;
    }
    if !s.contains('0') {
        return false;
    }

    let mut islands = vec![];
    for island in s.split('1') {
        if island.is_empty() {
            continue;
        }
        islands.push(island.len());
    }

    let len = islands.len();
    islands.iter().all(|&island| island == len)
}
