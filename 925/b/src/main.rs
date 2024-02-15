use std::io;

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for nl in lines.chunks(2) {
        let [_n, l] = &nl[..] else { panic!() };
        let mut nums: Vec<u32> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let ans = soln(&mut nums);
        println!("{}", display(ans));
    }
}

fn display(ans: bool) -> &'static str {
    if ans {
        "YES"
    } else {
        "NO"
    }
}

fn soln(buckets: &mut [u32]) -> bool {
    let n = buckets.len();
    let sum: u32 = buckets.iter().sum();
    assert_eq!(sum % n as u32, 0);
    let target = sum / n as u32;

    assert_ne!(n, 0);
    for i in 0..n - 1 {
        if buckets[i] < target {
            return false;
        }
        let extra = buckets[i] - target;
        buckets[i] = target;
        buckets[i + 1] += extra;
    }
    assert_eq!(buckets[n - 1], target);

    true
}
