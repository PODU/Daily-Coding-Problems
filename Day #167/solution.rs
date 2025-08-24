// Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
use std::collections::{HashMap, HashSet};

fn is_palin(s: &[u8], mut i: i32, mut j: i32) -> bool {
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
    let mut rev: HashMap<String, usize> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        rev.insert(w.chars().rev().collect(), i);
    }

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut res: Vec<(usize, usize)> = Vec::new();
    let mut add = |a: usize, b: usize, seen: &mut HashSet<(usize, usize)>, res: &mut Vec<(usize, usize)>| {
        if seen.insert((a, b)) {
            res.push((a, b));
        }
    };

    for (i, w) in words.iter().enumerate() {
        let bytes = w.as_bytes();
        let n = w.len() as i32;
        for cut in 0..=n {
            if is_palin(bytes, 0, cut - 1) {
                let suf = &w[cut as usize..];
                if let Some(&j) = rev.get(suf) {
                    if j != i {
                        add(j, i, &mut seen, &mut res);
                    }
                }
            }
            if cut < n && is_palin(bytes, cut, n - 1) {
                let pre = &w[..cut as usize];
                if let Some(&j) = rev.get(pre) {
                    if j != i {
                        add(i, j, &mut seen, &mut res);
                    }
                }
            }
        }
    }

    res.sort();
    let parts: Vec<String> = res.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("[{}]", parts.join(", "));
}
