// Day 213: Generate all valid IP addresses from a digit string.
// Approach: backtracking over the 3 dot positions; each segment 1-3 digits, 0-255, no leading zeros.
// Time O(1) effectively (length <= 12), Space O(1).
fn valid(seg: &str) -> bool {
    if seg.is_empty() || seg.len() > 3 {
        return false;
    }
    if seg.len() > 1 && seg.starts_with('0') {
        return false;
    }
    seg.parse::<u32>().map_or(false, |v| v <= 255)
}

fn solve(s: &str, start: usize, parts: &mut Vec<String>, res: &mut Vec<String>) {
    if parts.len() == 4 {
        if start == s.len() {
            res.push(parts.join("."));
        }
        return;
    }
    for len in 1..=3 {
        if start + len > s.len() {
            break;
        }
        let seg = &s[start..start + len];
        if valid(seg) {
            parts.push(seg.to_string());
            solve(s, start + len, parts, res);
            parts.pop();
        }
    }
}

fn restore(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    solve(s, 0, &mut Vec::new(), &mut res);
    res
}

fn main() {
    let r = restore("2542540123");
    let quoted: Vec<String> = r.iter().map(|x| format!("'{}'", x)).collect();
    println!("[{}]", quoted.join(", "));
}
