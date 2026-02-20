// Day 1104: Phone digit -> letters combinations via backtracking.
// Time: O(prod of choices * len). Space: O(len) recursion.
use std::collections::HashMap;

fn letter_combos(mapping: &HashMap<char, Vec<char>>, digits: &str) -> Vec<String> {
    let mut out = Vec::new();
    if digits.is_empty() {
        return out;
    }
    let chars: Vec<char> = digits.chars().collect();
    fn dfs(i: usize, chars: &[char], mapping: &HashMap<char, Vec<char>>,
           cur: &mut String, out: &mut Vec<String>) {
        if i == chars.len() {
            out.push(cur.clone());
            return;
        }
        for &c in &mapping[&chars[i]] {
            cur.push(c);
            dfs(i + 1, chars, mapping, cur, out);
            cur.pop();
        }
    }
    let mut cur = String::new();
    dfs(0, &chars, mapping, &mut cur, &mut out);
    out
}

fn main() {
    let mut mapping = HashMap::new();
    mapping.insert('2', vec!['a', 'b', 'c']);
    mapping.insert('3', vec!['d', 'e', 'f']);
    println!("{:?}", letter_combos(&mapping, "23"));
    // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
}
