use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let nums: Vec<u32> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();

        assert_eq!(nums.len(), 2);
        let num_ones = nums[0];
        let num_twos = nums[1];

        if can_zero(num_ones, num_twos) {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    assert!(lines.next().is_none());
}

fn can_zero(num_ones: u32, num_twos: u32) -> bool {
    num_ones % 2 == 0 && (num_twos % 2 == 0 || num_ones >= 2)
}

#[test]
fn test() {
    for (num_ones, num_twos, expected) in [
        (0, 0, true),
        (0, 1, false),
        (1, 0, false),
        (1, 1, false),
        (2, 0, true),
        (2, 1, true),
        (0, 2, true),
        (1, 2, false),
        (2, 2, true),
        (3, 2, false),
        (2, 3, true),
    ] {
        let actual = can_zero(num_ones, num_twos);
        assert_eq!(expected, actual);
    }
}
