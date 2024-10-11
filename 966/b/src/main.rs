use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let l = lines.next().unwrap().unwrap();
        let mut nums: Vec<usize> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(nums.len(), n);

        // 0-based indexing, please.
        for i in &mut nums {
            *i -= 1;
        }

        let ans = soln(&nums);
        if ans {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn soln(nums: &[usize]) -> bool {
    let n = nums.len();
    let mut filled = vec![false; n];

    filled[nums[0]] = true;

    for &i in &nums[1..] {
        let left = i != 0 && filled[i - 1];
        let right = i != n - 1 && filled[i + 1];
        let ok = left || right;

        if !ok {
            return false;
        }

        filled[i] = true;
    }

    true
}
