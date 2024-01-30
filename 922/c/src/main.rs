use std::io;

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for l in lines {
        let abr: Vec<u64> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let &[a, b, r] = &abr[..] else { panic!() };
        let ans = soln(a, b, r);
        println!("{ans}");
    }
}

fn soln(mut a: u64, mut b: u64, mut r: u64) -> u64 {
    // wlog, a > b
    (a, b) = if a > b { (a, b) } else { (b, a) };

    let mut first_diff = true;
    for i in (0..64).rev() {
        let ai = get(a, i);
        let bi = get(b, i);
        if ai == bi {
            continue;
        }
        if first_diff {
            first_diff = false;
            continue;
        }
        let bit = 1 << i;
        if r < bit {
            continue; // this bit is too large to flip
        }

        if ai && !bi {
            flip(&mut a, i);
            flip(&mut b, i);

            if msb(r) == Some(bit) {
                flip(&mut r, i);
            }
        }
    }

    ans(a, b)
}

fn msb(r: u64) -> Option<u64> {
    for i in (0..64).rev() {
        if get(r, i) {
            let bit = 1 << i;
            return Some(bit);
        }
    }
    None
}

fn get(a: u64, i: usize) -> bool {
    let bit = 1 << i;
    a & bit != 0
}

fn flip(a: &mut u64, i: usize) {
    let bit = 1 << i;
    *a ^= bit;
}

fn ans(a: u64, b: u64) -> u64 {
    a.abs_diff(b)
}
