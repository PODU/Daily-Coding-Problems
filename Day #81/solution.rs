// Day 81: Phone-number letter combinations via iterative cartesian product.
// Time O(prod of letters * len), Space O(output).
use std::collections::HashMap;

fn letter_combinations(mapping: &HashMap<char, Vec<&str>>, digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut res = vec![String::new()];
    for d in digits.chars() {
        let mut next = Vec::new();
        for prefix in &res {
            for letter in &mapping[&d] {
                next.push(format!("{}{}", prefix, letter));
            }
        }
        res = next;
    }
    res
}

fn main() {
    let mut mapping = HashMap::new();
    mapping.insert('2', vec!["a", "b", "c"]);
    mapping.insert('3', vec!["d", "e", "f"]);
    println!("{:?}", letter_combinations(&mapping, "23"));
    // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
}
