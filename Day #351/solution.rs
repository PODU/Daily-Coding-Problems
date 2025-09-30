// Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
// Time O(words * meanings * glossLen), Space O(vocab).
use std::collections::{BTreeMap, HashSet};

fn tok_set(s: &str) -> HashSet<String> {
    s.to_lowercase().split_whitespace().map(|w| w.to_string()).collect()
}

fn main() {
    let mut meanings: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    meanings.insert("bank", vec![
        "place where people deposit and withdraw money",
        "sloping land beside a river or lake of water",
    ]);
    meanings.insert("money", vec!["currency cash that people deposit"]);
    meanings.insert("river", vec!["large natural stream of water"]);

    let sentence = "I went to get money from the bank";
    let tokens: Vec<String> = sentence.to_lowercase().split_whitespace().map(|w| w.to_string()).collect();

    for (i, w) in tokens.iter().enumerate() {
        let senses = match meanings.get(w.as_str()) {
            Some(s) if s.len() >= 2 => s,
            _ => continue, // not ambiguous
        };
        let context: HashSet<&String> = tokens.iter().enumerate()
            .filter(|(j, _)| *j != i).map(|(_, t)| t).collect();
        let (mut best_idx, mut best_score) = (0usize, -1i32);
        for (idx, gloss) in senses.iter().enumerate() {
            let mut score = 0i32;
            for g in tok_set(gloss) {
                if context.contains(&g) { score += 1; }
            }
            if score > best_score { best_score = score; best_idx = idx; }
        }
        println!("{}: {}", w, senses[best_idx]);
    }
}
