// Day 57: Greedy word wrap into lines of length <= k. None if any word > k.
// Time: O(n), Space: O(n).
fn wrap(s: &str, k: usize) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut line = String::new();
    for word in s.split(' ') {
        if word.len() > k {
            return None;
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
        None => println!("null"),
        Some(out) => {
            let parts: Vec<String> = out.iter().map(|l| format!("\"{}\"", l)).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}
