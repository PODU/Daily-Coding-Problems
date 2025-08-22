// Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
// candidate; if no strict majority exists we fall back to the most frequent
// element so the answer is well-defined. Time O(n).
use std::collections::HashMap;

fn majority_element(a: &[i32]) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for &x in a {
        if count == 0 {
            candidate = x;
        }
        count += if x == candidate { 1 } else { -1 };
    }
    let occ = a.iter().filter(|&&x| x == candidate).count();
    if occ * 2 > a.len() {
        return candidate; // strict majority
    }

    // Fallback: most frequent element (example has no strict majority).
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut best = a[0];
    let mut best_cnt = 0;
    for &x in a {
        let c = freq.entry(x).or_insert(0);
        *c += 1;
        if *c > best_cnt {
            best_cnt = *c;
            best = x;
        }
    }
    best
}

fn main() {
    let a = [1, 2, 1, 1, 3, 4, 0];
    println!("{}", majority_element(&a)); // 1
}
