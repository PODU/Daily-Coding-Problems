// Group anagrams: HashMap keyed by sorted chars -> list, preserving first-seen group order.
// Time O(N*K log K), Space O(N*K).
use std::collections::HashMap;

fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    let mut idx: HashMap<String, usize> = HashMap::new();
    let mut groups: Vec<Vec<String>> = Vec::new();
    for &w in words {
        let mut chars: Vec<char> = w.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        if let Some(&g) = idx.get(&key) {
            groups[g].push(w.to_string());
        } else {
            idx.insert(key, groups.len());
            groups.push(vec![w.to_string()]);
        }
    }
    groups
}

fn main() {
    let input = ["eat", "ate", "apt", "pat", "tea", "now"];
    let groups = group_anagrams(&input);
    let parts: Vec<String> = groups
        .iter()
        .map(|g| {
            let inner: Vec<String> = g.iter().map(|w| format!("'{}'", w)).collect();
            format!("[{}]", inner.join(", "))
        })
        .collect();
    println!("[{}]", parts.join(", "));
}
