use std::io;

fn main() {
    let ans = precompute();

    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(n) = lines.next() {
        let n: usize = n.parse().unwrap();
        let ans = ans[n];
        println!("{ans}");
    }
}

/* idea
can't we just do the naive thing?
the longest number is 5 digits, so it's only a log factor on top of O(n)
Ok, let's try it.
*/

const N: usize = 200_000;

fn precompute() -> Vec<u64> {
    let mut ans = vec![0; N + 1];
    let mut sum = 0u64;
    for i in 1..=N {
        for c in i.to_string().chars() {
            let digit = c as u8 - b'0';
            sum += digit as u64;
        }
        ans[i] = sum;
    }
    ans
}
