// Day 798: Sentence equivalence via synonym pairs (NOT transitive).
// Store each unordered pair in a set; words match if equal or directly paired.
// Time O(W) per comparison, Space O(P).
use std::collections::HashSet;

fn key(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

fn same(syn: &HashSet<(String, String)>, a: &str, b: &str) -> bool {
    a == b || syn.contains(&key(a, b))
}

fn equivalent(syn: &HashSet<(String, String)>, s1: &[&str], s2: &[&str]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    s1.iter().zip(s2).all(|(x, y)| same(syn, x, y))
}

fn main() {
    let mut syn = HashSet::new();
    syn.insert(key("big", "large"));
    syn.insert(key("eat", "consume"));
    let a: Vec<&str> = "He wants to eat food.".split(' ').collect();
    let b: Vec<&str> = "He wants to consume food.".split(' ').collect();
    println!("{}", if equivalent(&syn, &a, &b) { "True (equivalent)" } else { "False" });
}
