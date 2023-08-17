use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nmd: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, m, d] = nmd.as_slice() else {
            Err("expected 3 nums: n m d")?
        };

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), m as usize);

        let (a1, a2) = soln(&nums, n, d);
        println!("{a1} {a2}");
    }

    Ok(())
}

// return smallest value after merging some pair of adj gaps,
// and how many different ways you can do this.
fn soln(nums: &[u64], n: u64, d: u64) -> (u64, usize) {
    assert!(2 <= n && n <= 10u64.pow(9));
    assert!(2 <= d && d <= n);
    assert!(2 <= nums.len() && nums.len() <= n as usize);
    assert!(nums.len() <= 10usize.pow(5));
    assert!(nums.iter().all(|&x| 1 <= x && x <= n));
    assert!(nums.windows(2).all(|pair| pair[0] < pair[1]));

    let prefix = if nums[0] != 1 { vec![1] } else { vec![] };
    let suffix = vec![n + 1];

    let nums2: Vec<_> = prefix.iter().chain(nums).chain(&suffix).copied().collect();
    let gaps: Vec<_> = nums2.windows(2).map(|pair| pair[1] - pair[0]).collect();

    // dbg!(&gaps);

    // one for the cookie stand, plus cookies along the way
    // let gap_score = |g| 1 + (g - 1) / d;
    let ceil_div = |a, b| {
        assert_ne!(b, 0);
        let extra = if a % b != 0 { 1 } else { 0 };
        a / b + extra
    };
    let gap_score = |g| ceil_div(g, d);

    let score: u64 = gaps.iter().map(|&g| gap_score(g)).sum();

    // dbg!(score);

    // compute min and num ways to hit min

    let mut best = u64::MAX;
    let mut count = 0;

    for i in 0..gaps.len() - 1 {
        // merge gaps [i, i+1]

        let mut score2 = score;
        score2 -= gap_score(gaps[i]);
        score2 -= gap_score(gaps[i + 1]);
        score2 += gap_score(gaps[i] + gaps[i + 1]);
        // dbg!(gaps[i], gaps[i + 1], score2);

        if score2 < best {
            best = score2;
            count = 1;
        } else if score2 == best {
            count += 1;
        }
    }

    // edge-case: removing the first cookie stand does nothing;
    // might need to include in count
    if nums[0] == 1 && best == score {
        count += 1;
    }

    (best, count)
}
