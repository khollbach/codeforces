use std::collections::HashMap;
use std::hash::Hash;
use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let l = lines.next().unwrap().unwrap();
        let a: Vec<i32> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);

        let l = lines.next().unwrap().unwrap();
        let m: usize = l.parse().unwrap();

        for _ in 0..m {
            let s = lines.next().unwrap().unwrap();

            let ans = is_isomorphic(&a, &s);
            if ans {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

fn is_isomorphic(a: &[i32], s: &str) -> bool {
    if a.len() != s.len() {
        return false;
    }

    let a_groups = sets_of_indices(a);
    let s_groups = sets_of_indices(s.as_bytes());
    a_groups == s_groups
}

fn sets_of_indices<T: Hash + Eq>(a: &[T]) -> Vec<Vec<usize>> {
    let mut mapping = HashMap::<&T, Vec<usize>>::new();
    for (i, x) in a.iter().enumerate() {
        mapping.entry(x).or_default().push(i);
    }

    let mut groups: Vec<_> = mapping.into_values().collect();
    groups.sort();
    groups
}
