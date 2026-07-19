// Day 1842: Majority / most-frequent element via a frequency count.
// (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).
use std::collections::HashMap;

fn majority(a: &[i32]) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut best = a[0];
    let mut best_count = 0;
    for &x in a {
        let c = freq.entry(x).or_insert(0);
        *c += 1;
        if *c > best_count {
            best_count = *c;
            best = x;
        }
    }
    best
}

fn main() {
    println!("{}", majority(&[1, 2, 1, 1, 3, 4, 0])); // 1
}
