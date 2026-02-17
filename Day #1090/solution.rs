// Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
// Time O(n*k^2), Space O(n*k).
use std::collections::{BTreeSet, HashMap};

fn is_pal(s: &[u8], mut i: i32, mut j: i32) -> bool {
    while i < j {
        if s[i as usize] != s[j as usize] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn main() {
    let words = vec!["code", "edoc", "da", "d"];
    let mut d = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        d.insert(*w, i);
    }
    let mut res: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (i, w) in words.iter().enumerate() {
        let b = w.as_bytes();
        let l = b.len();
        for j in 0..=l {
            if is_pal(b, 0, j as i32 - 1) {
                let r: String = w[j..].chars().rev().collect();
                if let Some(&k) = d.get(r.as_str()) {
                    if k != i {
                        res.insert((k, i));
                    }
                }
            }
            if j != l && is_pal(b, j as i32, l as i32 - 1) {
                let ll: String = w[..j].chars().rev().collect();
                if let Some(&k) = d.get(ll.as_str()) {
                    if k != i {
                        res.insert((i, k));
                    }
                }
            }
        }
    }
    let parts: Vec<String> = res.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("[{}]", parts.join(", "));
}
