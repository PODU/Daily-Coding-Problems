// Greedy line wrapping: fit max words per line within width k.
// Return None if any single word exceeds k.
// Time: O(n), Space: O(n).

fn wrap(s: &str, k: usize) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in s.split_whitespace() {
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
        Some(out) => {
            let quoted: Vec<String> = out.iter().map(|l| format!("\"{}\"", l)).collect();
            println!("[{}]", quoted.join(", "));
            // ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
        }
        None => println!("null"),
    }
}
