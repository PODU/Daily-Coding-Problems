// Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
// Approach: greedy single pass over words. O(n) time, O(n) space. None if a word > k.

fn wrap(s: &str, k: usize) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in s.split(' ') {
        if word.len() > k {
            return None; // impossible
        }
        if line.is_empty() {
            line = word.to_string();
        } else if line.len() + 1 + word.len() <= k {
            line.push(' ');
            line.push_str(word);
        } else {
            out.push(line);
            line = word.to_string();
        }
    }
    if !line.is_empty() {
        out.push(line);
    }
    Some(out)
}

fn main() {
    match wrap("the quick brown fox jumps over the lazy dog", 10) {
        Some(out) => {
            let parts: Vec<String> = out.iter().map(|l| format!("\"{}\"", l)).collect();
            println!("[{}]", parts.join(", "));
        }
        None => println!("null"),
    }
}
