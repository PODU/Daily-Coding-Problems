// Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
// Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").
fn valid(seg: &str) -> bool {
    if seg.is_empty() || seg.len() > 3 {
        return false;
    }
    if seg.len() > 1 && seg.starts_with('0') {
        return false;
    }
    seg.parse::<u32>().map_or(false, |n| n <= 255)
}

fn bt(s: &[u8], start: usize, part: usize, cur: &mut Vec<String>, res: &mut Vec<String>) {
    if part == 4 {
        if start == s.len() {
            res.push(cur.join("."));
        }
        return;
    }
    let mut len = 1;
    while len <= 3 && start + len <= s.len() {
        let seg = std::str::from_utf8(&s[start..start + len]).unwrap();
        if valid(seg) {
            cur.push(seg.to_string());
            bt(s, start + len, part + 1, cur, res);
            cur.pop();
        }
        len += 1;
    }
}

fn restore(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    bt(s.as_bytes(), 0, 0, &mut Vec::new(), &mut res);
    res
}

fn main() {
    let res = restore("2542540123");
    let parts: Vec<String> = res.iter().map(|x| format!("'{}'", x)).collect();
    println!("[{}]", parts.join(", "));
}
