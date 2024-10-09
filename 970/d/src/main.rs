use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let l = lines.next().unwrap().unwrap();
        let mut p: Vec<usize> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(p.len(), n);

        // Account for obo.
        for x in &mut p {
            *x -= 1;
        }

        let l = lines.next().unwrap().unwrap();
        let is_black: Vec<bool> = l
            .chars()
            .map(|c| match c {
                '0' => true,
                '1' => false,
                _ => panic!("{c}"),
            })
            .collect();
        assert_eq!(is_black.len(), n);

        let ans = reachable_black(&p, &is_black);
        for x in ans {
            print!("{x} ");
        }
        println!();
    }
}

fn reachable_black(p: &[usize], is_black: &[bool]) -> Vec<u32> {
    let n = p.len();
    assert_eq!(is_black.len(), n);

    let mut out = vec![None; n];

    for i in 0..n {
        if out[i].is_some() {
            continue;
        }

        // traverse the cycle once; counting blacks
        let mut count = 0;
        let mut j = i;
        loop {
            if is_black[j] {
                count += 1;
            }
            j = p[j];
            if j == i {
                break;
            }
        }

        // traverse again, marking output values
        let mut j = i;
        loop {
            out[j] = Some(count);
            j = p[j];
            if j == i {
                break;
            }
        }
    }

    out.into_iter().map(Option::unwrap).collect()
}
