use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l) = lines.next() {
        let l = l.unwrap();
        let nx: Vec<&str> = l.split_whitespace().collect();
        assert_eq!(nx.len(), 2);
        let n: usize = nx[0].parse().unwrap();
        let x: u64 = nx[1].parse().unwrap();

        let a: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);

        let ans = soln(&a, x);
        println!("{ans}");
    }
}

fn soln(a: &[u64], x: u64) -> usize {
    let n = a.len();
    let mut ways_to_end_here = vec![1; n + 1];

    let mut curr_sum = 0;
    let mut i = 0;
    let mut j = 0;

    while j < n {
        curr_sum += a[j];
        j += 1;

        while curr_sum > x {
            ways_to_end_here[j] += ways_to_end_here[i];

            curr_sum -= a[i];
            i += 1;
        }
    }

    let mut ways_to_die = 0;
    for w in ways_to_end_here {
        ways_to_die += w;
    }
    ways_to_die -= n + 1; // don't count empty slices

    let nonempty_slices = n * (n + 1) / 2;
    nonempty_slices - ways_to_die
}
