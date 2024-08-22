use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _n = lines.next().unwrap().unwrap();

    for l in lines {
        let l = l.unwrap();

        let nums: Vec<u32> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        let &[n, k] = nums.as_slice() else {
            panic!()
        };

        let ans = soln(n, k);
        println!("{}", ans);
    }
}

fn soln(n: u32, k: u32) -> u32 {
    (n - 1).div_ceil(k - 1)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(4, 5, 1)]
    #[test_case(121, 5, 30)]
    #[test_case(122, 5, 31)]
    #[test_case(123, 5, 31)]
    #[test_case(124, 5, 31)]
    #[test_case(125, 5, 31)]
    #[test_case(126, 5, 32)]
    #[test_case(7, 7, 1)]
    #[test_case(1, 2, 0)]
    #[test_case(2, 2, 1)]
    #[test_case(3, 2, 2)]
    #[test_case(4, 2, 3)]
    fn test(n: u32, k: u32, expected: u32) {
        let actual = super::soln(n, k);
        assert_eq!(expected, actual);
    }
}
