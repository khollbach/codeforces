use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let l = lines.next().unwrap().unwrap();
        let a: Vec<u64> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);

        let s = lines.next().unwrap().unwrap();
        assert_eq!(s.len(), n);

        let ans = soln(&a, &s);
        println!("{ans}");
    }
}

fn soln(a: &[u64], s: &str) -> u64 {
    let s = s.as_bytes();

    let n = a.len();
    assert_eq!(s.len(), n);
    assert_ne!(n, 0);

    let mut prefix_sums = vec![0; n + 1];
    for i in 0..n {
        prefix_sums[i + 1] = prefix_sums[i] + a[i];
    }

    let mut score = 0;

    let mut i = 0;
    let mut j = n as isize - 1; // inclusive

    loop {
        while i < n && s[i] == b'R' {
            i += 1;
        }
        while j >= 0 && s[j as usize] == b'L' {
            j -= 1;
        }

        if j <= i as isize {
            break;
        }

        let slice_sum = prefix_sums[j as usize + 1] - prefix_sums[i];
        score += slice_sum;

        i += 1;
        j -= 1;
    }

    score
}
