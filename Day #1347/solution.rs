// Reorganize string: count freqs, if max > (n+1)/2 impossible; sort chars by freq desc (tie by char),
// fill even indices first then odd. Time O(n log k), Space O(n).
use std::collections::BTreeMap;

fn reorganize(s: &str) -> Option<String> {
    let bytes: Vec<u8> = s.bytes().collect();
    let n = bytes.len();
    let mut cnt: BTreeMap<u8, usize> = BTreeMap::new();
    for &b in &bytes {
        *cnt.entry(b).or_insert(0) += 1;
    }
    for (_, &v) in &cnt {
        if v > (n + 1) / 2 {
            return None;
        }
    }
    let mut chars: Vec<(usize, u8)> = cnt.iter().map(|(&c, &f)| (f, c)).collect();
    chars.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1))); // freq desc, tie char asc
    let mut res = vec![0u8; n];
    let mut idx = 0usize;
    for (f, c) in chars {
        for _ in 0..f {
            res[idx] = c;
            idx += 2;
            if idx >= n {
                idx = 1;
            }
        }
    }
    Some(String::from_utf8(res).unwrap())
}

fn main() {
    match reorganize("aaabbc") {
        Some(r) => println!("{}", r),
        None => println!("None"),
    }
    match reorganize("aaab") {
        Some(r) => println!("{}", r),
        None => println!("None"),
    }
}
