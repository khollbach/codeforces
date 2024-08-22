use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for l in lines {
        let nxy: Vec<usize> = l
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let &[n, x, y] = nxy.as_slice() else { panic!() };

        let ans = soln(n, x, y);
        for x in ans {
            print!("{x} ");
        }
        println!();
    }
}

fn soln(n: usize, x: usize, y: usize) -> Vec<i8> {
    assert!(n >= 2);
    assert!(1 <= y && y < x && x <= n);

    // 0-based indexing.
    let x = x - 1;
    let y = y - 1;

    let middle_len = x - y + 1;
    let middle = vec![1; middle_len];

    let prefix_len = y;
    let mut prefix: Vec<_> = [-1, 1].into_iter().cycle().take(prefix_len).collect();
    prefix.reverse();

    let suffix_len = n - x - 1;
    let suffix: Vec<_> = [-1, 1].into_iter().cycle().take(suffix_len).collect();

    let mut answer = Vec::with_capacity(n);
    answer.extend_from_slice(&prefix);
    answer.extend_from_slice(&middle);
    answer.extend_from_slice(&suffix);
    answer
}
