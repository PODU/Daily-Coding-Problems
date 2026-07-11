// Word wrap greedily: pack max words per line <= k, return None if any word > k. O(total length) time.
fn word_wrap(s: &str, k: usize) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();
    for w in s.split(' ') {
        if w.len() > k {
            return None;
        }
        if cur.is_empty() {
            cur = w.to_string();
        } else if cur.len() + 1 + w.len() <= k {
            cur.push(' ');
            cur.push_str(w);
        } else {
            lines.push(cur);
            cur = w.to_string();
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    Some(lines)
}

fn main() {
    match word_wrap("the quick brown fox jumps over the lazy dog", 10) {
        None => println!("None"),
        Some(res) => {
            let parts: Vec<String> = res.iter().map(|ln| format!("\"{}\"", ln)).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}
