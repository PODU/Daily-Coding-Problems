// Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
// Time O(1) (bounded by string length <= 12), Space O(1) extra.
fn valid(seg: &str) -> bool {
    if seg.is_empty() || seg.len() > 3 {
        return false;
    }
    if seg.len() > 1 && seg.starts_with('0') {
        return false;
    }
    seg.parse::<u32>().map_or(false, |n| n <= 255)
}

fn backtrack(s: &str, start: usize, part: usize, cur: &mut Vec<String>, res: &mut Vec<String>) {
    if part == 4 {
        if start == s.len() {
            res.push(cur.join("."));
        }
        return;
    }
    let mut len = 1;
    while len <= 3 && start + len <= s.len() {
        let seg = &s[start..start + len];
        if valid(seg) {
            cur.push(seg.to_string());
            backtrack(s, start + len, part + 1, cur, res);
            cur.pop();
        }
        len += 1;
    }
}

fn main() {
    let s = "2542540123";
    let mut res = Vec::new();
    backtrack(s, 0, 0, &mut Vec::new(), &mut res);
    let parts: Vec<String> = res.iter().map(|r| format!("'{}'", r)).collect();
    println!("[{}]", parts.join(", "));
}
