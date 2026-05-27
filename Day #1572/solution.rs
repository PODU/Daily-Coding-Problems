// Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.
use std::collections::HashMap;

fn letter_combinations(digits: &str, mp: &HashMap<char, &str>) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }
    let mut res = vec![String::new()];
    for d in digits.chars() {
        let mut next = Vec::new();
        for pre in &res {
            for c in mp[&d].chars() {
                next.push(format!("{}{}", pre, c));
            }
        }
        res = next;
    }
    res
}

fn main() {
    let mut mp = HashMap::new();
    mp.insert('2', "abc");
    mp.insert('3', "def");
    let res = letter_combinations("23", &mp);
    let parts: Vec<String> = res.iter().map(|w| format!("\"{}\"", w)).collect();
    println!("[{}]", parts.join(", "));
}
