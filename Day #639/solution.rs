// Day 639: Letter combinations of a phone number.
// Approach: iterative Cartesian product over digit->letters map.
// Time: O(4^n * n), Space: O(4^n).
use std::collections::HashMap;

fn letter_combinations(digits: &str, mapping: &HashMap<char, Vec<&str>>) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut res = vec![String::new()];
    for d in digits.chars() {
        let mut next = Vec::new();
        for prefix in &res {
            for ch in &mapping[&d] {
                next.push(format!("{}{}", prefix, ch));
            }
        }
        res = next;
    }
    res
}

fn main() {
    let mut mapping: HashMap<char, Vec<&str>> = HashMap::new();
    mapping.insert('2', vec!["a", "b", "c"]);
    mapping.insert('3', vec!["d", "e", "f"]);
    mapping.insert('4', vec!["g", "h", "i"]);
    mapping.insert('5', vec!["j", "k", "l"]);
    mapping.insert('6', vec!["m", "n", "o"]);
    mapping.insert('7', vec!["p", "q", "r", "s"]);
    mapping.insert('8', vec!["t", "u", "v"]);
    mapping.insert('9', vec!["w", "x", "y", "z"]);
    println!("{:?}", letter_combinations("23", &mapping));
}
