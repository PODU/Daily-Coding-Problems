// Sort characters by descending frequency (ties: first-occurrence order).
// Time: O(n log d), Space: O(n).
use std::collections::HashMap;

fn frequency_sort(s: &str) -> String {
    let mut cnt: HashMap<char, usize> = HashMap::new();
    let mut first: HashMap<char, usize> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        *cnt.entry(c).or_insert(0) += 1;
        first.entry(c).or_insert(i);
    }
    let mut chars: Vec<char> = cnt.keys().copied().collect();
    chars.sort_by(|&a, &b| {
        cnt[&b].cmp(&cnt[&a]).then(first[&a].cmp(&first[&b]))
    });
    chars.iter().map(|&c| c.to_string().repeat(cnt[&c])).collect()
}

fn main() {
    println!("{}", frequency_sort("tweet"));
}
