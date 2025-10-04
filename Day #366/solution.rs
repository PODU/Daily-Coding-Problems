// Day 366: Rearrange so no two adjacent chars match (else null).
// Greedy with a max-heap by frequency; always place the most frequent char that
// isn't the one just placed. Feasible iff maxFreq <= (n+1)/2. Time O(n log k).
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn reorganize(s: &str) -> Option<String> {
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    let mut heap: BinaryHeap<(i32, char)> = cnt.into_iter().map(|(c, n)| (n, c)).collect();
    let mut res = String::new();
    let mut prev: Option<(i32, char)> = None;
    while let Some((n, c)) = heap.pop() {
        res.push(c);
        if let Some(p) = prev {
            if p.0 > 0 {
                heap.push(p);
            }
        }
        prev = Some((n - 1, c));
    }
    if res.len() == s.len() {
        Some(res)
    } else {
        None
    }
}

fn main() {
    println!("{}", reorganize("yyz").unwrap_or_else(|| "null".into())); // yzy
    println!("{}", reorganize("yyy").unwrap_or_else(|| "null".into())); // null
}
