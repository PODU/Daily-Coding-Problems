// Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
use std::collections::HashSet;

fn longest_consecutive(a: &[i64]) -> usize {
    let s: HashSet<i64> = a.iter().copied().collect();
    let mut best = 0;
    for &x in &s {
        if s.contains(&(x - 1)) {
            continue;
        }
        let mut len = 1;
        let mut cur = x;
        while s.contains(&(cur + 1)) {
            cur += 1;
            len += 1;
        }
        best = best.max(len);
    }
    best
}

fn main() {
    println!("{}", longest_consecutive(&[100, 4, 200, 1, 3, 2]));
}
