// Sentence similarity (non-transitive). Store synonym pairs both directions in a set; compare word by word.
// Time O(words + pairs), Space O(pairs).
use std::collections::HashSet;

fn are_similar(s1: &[String], s2: &[String], pairs: &[(&str, &str)]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut syn: HashSet<(String, String)> = HashSet::new();
    for (a, b) in pairs {
        syn.insert((a.to_string(), b.to_string()));
        syn.insert((b.to_string(), a.to_string()));
    }
    for (w1, w2) in s1.iter().zip(s2.iter()) {
        if w1 == w2 {
            continue;
        }
        if syn.contains(&(w1.clone(), w2.clone())) {
            continue;
        }
        return false;
    }
    true
}

fn tokenize(s: &str) -> Vec<String> {
    s.replace('.', "")
        .split_whitespace()
        .map(|t| t.to_string())
        .collect()
}

fn main() {
    let synonyms = [("big", "large"), ("eat", "consume")];
    let a = tokenize("He wants to eat food.");
    let b = tokenize("He wants to consume food.");
    println!("{}", if are_similar(&a, &b, &synonyms) { "True" } else { "False" });
}
