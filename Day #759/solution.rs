// Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
// At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).
fn valid_octet(s: &str) -> bool {
    if s.is_empty() || s.len() > 3 {
        return false;
    }
    if s.len() > 1 && s.starts_with('0') {
        return false;
    }
    s.parse::<u32>().map_or(false, |v| v <= 255)
}

fn backtrack(s: &str, start: usize, parts: &mut Vec<String>, res: &mut Vec<String>) {
    if parts.len() == 4 {
        if start == s.len() {
            res.push(parts.join("."));
        }
        return;
    }
    let mut len = 1;
    while len <= 3 && start + len <= s.len() {
        let seg = &s[start..start + len];
        if valid_octet(seg) {
            parts.push(seg.to_string());
            backtrack(s, start + len, parts, res);
            parts.pop();
        }
        len += 1;
    }
}

fn restore_ips(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut parts = Vec::new();
    backtrack(s, 0, &mut parts, &mut res);
    res
}

fn main() {
    let res = restore_ips("2542540123");
    let quoted: Vec<String> = res.iter().map(|r| format!("'{}'", r)).collect();
    println!("[{}]", quoted.join(", "));
    // ['254.25.40.123', '254.254.0.123']
}
