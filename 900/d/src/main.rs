use std::{collections::BTreeMap, error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nk: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            Err("nk")?
        };

        let s = lines.next().ok_or("s")??;
        assert_eq!(s.len(), n);

        let l: Vec<usize> = lines
            .next()
            .ok_or("l")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(l.len(), k);

        let r: Vec<usize> = lines
            .next()
            .ok_or("r")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(r.len(), k);

        let intervals: Vec<_> = zip(l, r).map(|(i_1_based, j)| (i_1_based - 1, j)).collect();

        let q = lines.next().ok_or("q")??.parse()?;
        let mut queries: Vec<_> = lines
            .next()
            .ok_or("queries")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(queries.len(), q);
        for x in &mut queries {
            *x -= 1; // 0-based, please!
        }

        let ans = soln(s, &intervals, &queries);
        println!("{ans}");
    }

    Ok(())
}

/*

split the string into chunks

apply queries to the chunks
* a query records itself at the corr index
    (account for mirroring)

resolve queries on each chunk
* this means walking the chunk left-to-right and
    performing mirror-swaps on some indices
    (remember to include (xor) previous swaps as you go,
     since they subsume smaller swaps)

stitch the chunks together

*/

fn soln(s: String, intervals: &[(usize, usize)], queries: &[usize]) -> String {
    // dbg!(&s, intervals, queries);
    let s = s.into_bytes();

    let mut chunks = BTreeMap::new();
    for &(i, j) in intervals {
        let slice = s[i..j].to_vec();
        chunks.insert(
            i,
            Chunk {
                i,
                j,
                num_flips: vec![false; ceil_div(slice.len(), 2)],
                slice,
            },
        );
    }

    for &x in queries {
        let (_i, chunk) = chunks.range_mut(..=x).next_back().unwrap();
        chunk.record_query(x);
    }

    for c in chunks.values_mut() {
        c.apply_queries();
    }

    let s: Vec<u8> = chunks.into_values().flat_map(|ch| ch.slice).collect();
    String::from_utf8(s).unwrap()
}

#[derive(Debug)]
struct Chunk {
    slice: Vec<u8>,
    i: usize,
    j: usize,
    num_flips: Vec<bool>, // ceil(s.len() / 2)
}

fn ceil_div(x: usize, y: usize) -> usize {
    let extra = if x % y != 0 { 1 } else { 0 };
    x / y + extra
}

impl Chunk {
    fn record_query(&mut self, idx: usize) {
        assert!(self.i <= idx && idx < self.j);

        // canonicalize
        let mid = self.i + (self.j - self.i) / 2;
        let offset = if idx >= mid {
            let delta = self.j - 1 - idx;
            delta
        } else {
            idx - self.i
        };

        self.num_flips[offset] ^= true;
    }

    fn apply_queries(&mut self) {
        let mut prev = false;
        for o in 0..self.num_flips.len() {
            self.num_flips[o] ^= prev; // account for subsumption
            if self.num_flips[o] {
                let len = self.slice.len();
                self.slice.swap(o, len - 1 - o);
            }

            prev = self.num_flips[o];
        }
    }
}
