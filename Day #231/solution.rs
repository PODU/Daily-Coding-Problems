// Reorganize string: greedily place the most frequent remaining char that differs from the last.
// Max-heap by count. Time: O(n log A), Space: O(A).
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn reorganize(s: &str) -> Option<String> {
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    // highest count first; ties broken by smallest char for determinism
    let mut heap: BinaryHeap<(i32, Reverse<char>)> =
        cnt.into_iter().map(|(ch, c)| (c, Reverse(ch))).collect();
    let mut res = String::new();
    let mut prev: Option<(i32, Reverse<char>)> = None;
    while let Some((c, Reverse(ch))) = heap.pop() {
        res.push(ch);
        if let Some(p) = prev {
            if p.0 > 0 {
                heap.push(p);
            }
        }
        prev = Some((c - 1, Reverse(ch)));
    }
    if res.len() == s.len() {
        Some(res)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", reorganize("aaabbc")); // Some("ababac") (a valid arrangement)
    println!("{:?}", reorganize("aaab"));   // None
}
