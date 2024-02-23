use std::io;

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(nm) = lines.next() {
        let nm: Vec<u32> = nm.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let &[_n, m] = &nm[..] else { panic!() };
        let nums: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let commands = lines.next().unwrap();
        let ans = soln(&nums, &commands, m);
        for x in ans {
            print!("{x} ");
        }
        println!();
    }
}

/* new idea
since there are fewer possible buckets than input elements, we'll do this:
(.... but does it work?)
*/

fn soln(mut nums: &[u32], commands: &str, m: u32) -> Vec<u32> {
    let mut buckets = vec![0; 10_usize.pow(4) + 1];
    for &x in nums {
        buckets[x as usize] += 1;
    }

    let mut out = vec![];
    for c in commands.chars() {
        let mut product = 1; // isn't this also linear in the worst case?
        for x in 1..=10_usize.pow(4) {
            for _ in 0..buckets[x] {
                product *= x as u32;
                product %= m; // wait a second.. this approach doesn't do what i want
            }
        }
        out.push(product);

        match c {
            'L' => nums = &nums[1..],
            'R' => nums = &nums[..nums.len() - 1],
            _ => panic!("invalid command: {c:?}"),
        }
    }
    out
}

// struct BigNum {
//     factors: Vec<u32>,
// }

// impl BigNum {
//     fn new(n: u32, m: u32) -> Self {
//         assert_ne!(n, 0);
//         assert_ne!(m, 0);
//         Self { factors: factor(n, m) }
//     }

//     fn m(&self) -> u32 {
//         self.factors.len() as u32
//     }

//     /// O(m)
//     fn mul_assign(&mut self, other: &Self) {
//         assert_eq!(self.m(), other.m());
//         for i in 0..self.m() {
//             self.factors[i] += other.factors[i];
//         }
//     }

//     /// O(m)
//     fn div_assign(&mut self, other: &Self) {
//         assert_eq!(self.m(), other.m());
//         for i in 0..self.m() {
//             assert!(self.factors[i] >= other.factors[i]);
//             self.factors[i] -= other.factors[i];
//         }
//     }

//     /// O(m)
//     fn modulus(&self) -> u32 {
//         let mut out = 1;
//         for i in 0..self.m() {
//             self.factors[i]
//         }
//     }
// }

/// O(sqrt(n) + m)
fn factor(mut n: u32, m: u32) -> Vec<u32> {
    assert_ne!(n, 0);
    assert_ne!(m, 0);
    let mut out = vec![0; m as usize];
    for i in 2.. {
        while n % i == 0 {
            out[i % m] += 1;
            n /= i;
        }
        // early stopping: we don't need to check all the numbers greater than sqrt(n),
        // we can just skip to the part where i=n and be done with it.
        if i * i > n {
            out[n % m] += 1;
            break;
        }
    }
    out
}

/* idea:

[ 3 1 4 2 ]   => 24  /3 
    ^   ^
              => 8
LRRL



compute all prefix products and suffix products, as "bignums"
(bignum object stores a multiset of prime factors)
iterate over the commands list; hold two pointers (current slice of the input)
for each state (before executing the command),
    output the overall product divided by the prefix product, divided by the suffix product

UPDATE: the above is accidentally quadratic if you compute prefix/suffix products up front.
Instead we want to just start with a total product and then update as we execute commands.
*/

// fn soln(mut nums: &[u32], commands: &str) -> u32 {
//     let mut total_product = BigNum::one();
//     for &x in nums {
//         total_product.mul_assign(BigNum::new(x));
//     }
//     let mut curr_product = total_product;

//     assert_eq!(nums.len(), commands.len());
//     for c in commands.chars() {
//         out.push(curr_product); // isn't this also linear in the worst case?
//         match c {
//             "L" => (),
//             "R" => (),
//             _ => panic!("invalid command: {c:?}"),
//         }
//     }
// }
