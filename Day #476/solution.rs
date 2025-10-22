// Find duplicate in array of n+1 ints from 1..n using a HashSet.
// Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
use std::collections::HashSet;

fn find_duplicate(a: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    for &x in a {
        if !seen.insert(x) {
            return x;
        }
    }
    -1
}

fn main() {
    println!("{}", find_duplicate(&[1, 3, 4, 2, 2]));
}
