use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l1) = lines.next() {
        let _num_bits: u32 = l1.unwrap().parse().unwrap();

        let l2 = lines.next().unwrap().unwrap();
        let bits: Vec<_> = l2
            .chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!(),
            })
            .collect();

        let ans = soln(&bits);
        if ans {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn soln(bits: &[bool]) -> bool {
    let ones = count_ones(bits);
    let zeros = count_zero_islands(bits);
    ones > zeros
}

fn count_ones(bits: &[bool]) -> usize {
    let mut count = 0;
    for &b in bits {
        if b {
            count += 1;
        }
    }
    count
}

fn count_zero_islands(bits: &[bool]) -> usize {
    let mut count = 0;

    for i in 0..bits.len() {
        // If the previous value is 0, skip this value.
        // (That way we only end up counting the first 0 of each 0-island.)
        if i != 0 && !bits[i - 1] {
            continue;
        }

        if !bits[i] {
            count += 1;
        }
    }

    count
}
