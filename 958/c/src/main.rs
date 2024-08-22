use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for l in lines {
        let n: u64 = l.unwrap().parse().unwrap();

        let ans = soln(n);
        println!("{}", ans.len());
        for x in ans {
            print!("{} ", x);
        }
        println!();
    }
}

fn soln(n: u64) -> Vec<u64> {
    // Edge-case.
    if n.count_ones() == 1 {
        // We're only allowed to output *positive* numbers,
        // so output just [n] instead of [0, n].
        return vec![n];
    }

    let mut out = vec![];

    for i in (0..64).rev() {
        let bit = 1 << i;

        // Clear bit i.
        let mut candidate = n;
        candidate &= !bit;

        if candidate != n {
            out.push(candidate);
        }
    }

    out.push(n);

    out
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(0)]
    // #[test_case(1)]
    #[test_case(12345)]
    fn test(mut n: u64) {
        if n == 12345 {
            n = rand::random();
        }

        let expected_len = (n.count_ones() + 1) as usize;
        let actual = super::soln(n);
        assert_eq!(expected_len, actual.len());

        for i in 0..actual.len() {
            assert!(actual[i] <= n);

            if i != 0 {
                assert!(actual[i - 1] < actual[i]);
                assert_eq!(actual[i - 1] | actual[i], n);
            }
        }
    }
}
