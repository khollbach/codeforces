use std::{collections::HashSet, io};

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(n) = lines.next() {
        let n: usize = n.parse().unwrap();
        let rows: Vec<String> = lines.by_ref().take(n).collect();
        let ans = soln(&rows);
        println!("{ans}");
    }
}

/* idea
for each row, count the number of ones
if non-zero, put the freq in a hashset

if the hashset has len 1, then it's a square,
o/w len>1 and its a triangle
*/

fn soln(rows: &[String]) -> &'static str {
    let mut widths = HashSet::new();
    for r in rows {
        let num_ones = r.chars().filter(|&c| c == '1').count();
        if num_ones > 0 {
            widths.insert(num_ones);
        }
    }
    assert!(widths.len() > 0);
    if widths.len() == 1 {
        "SQUARE"
    } else {
        "TRIANGLE"
    }
}
