use std::{error::Error, io, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let ndk: Vec<usize> = lines
            .next()
            .ok_or("ndk")??
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, d, k] = ndk.as_slice() else {
            panic!("expected 3 nums ndk, got {}", ndk.len());
        };

        let jobs: Vec<Interval> = lines
            .by_ref()
            .take(k)
            .map(|l| l?.parse())
            .collect::<Result<_>>()?;
        assert_eq!(jobs.len(), k);

        let (b, m) = brother_mother_visits(n, &jobs, d);
        println!("{} {}", b + 1, m + 1); // 1-based indexing.
    }

    assert!(lines.next().is_none());
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: usize,
    /// Inclusive.
    end: usize,
}

impl FromStr for Interval {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let nums: Vec<usize> = s
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[start, end] = nums.as_slice() else {
            return Err(format!("expected 2 nums got {:?}", nums.len()).into());
        };

        // 0-based indexing, please.
        let start = start.checked_sub(1).ok_or("start 0")?;
        let end = end.checked_sub(1).ok_or("end 0")?;

        Ok(Self { start, end })
    }
}

fn brother_mother_visits(
    num_days: usize,
    jobs: &[Interval],
    visit_duration: usize,
) -> (usize, usize) {
    assert!(num_days > 0);
    assert!(visit_duration > 0);
    assert!(visit_duration <= num_days);
    let k = jobs.len();

    let mut starts = vec![0; num_days];
    let mut ends = vec![0; num_days];
    for job in jobs {
        starts[job.start] += 1;
        ends[job.end] += 1;
    }

    let mut ends_before = vec![0; num_days];
    for i in 1..num_days {
        ends_before[i] = ends_before[i - 1] + ends[i - 1];
    }

    let mut starts_after = vec![0; num_days];
    for i in (0..num_days - 1).rev() {
        starts_after[i] = starts_after[i + 1] + starts[i + 1];
    }

    let mut min = None;
    let mut min_idx = None;

    let mut max = None;
    let mut max_idx = None;

    for i in 0..num_days - visit_duration + 1 {
        let strictly_before = ends_before[i];
        let strictly_after = starts_after[i + visit_duration - 1];

        let num_hit = k - strictly_before - strictly_after;

        if min.is_none() || num_hit < min.unwrap() {
            min = Some(num_hit);
            min_idx = Some(i);
        }
        if max.is_none() || num_hit > max.unwrap() {
            max = Some(num_hit);
            max_idx = Some(i);
        }
    }

    (max_idx.unwrap(), min_idx.unwrap())
}
