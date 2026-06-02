// Generate all valid IPv4 addresses by backtracking: place 3 dots, each
// segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
fn backtrack(s: &[u8], start: usize, part: usize, cur: &mut Vec<String>, res: &mut Vec<String>) {
    if part == 4 {
        if start == s.len() {
            res.push(cur.join("."));
        }
        return;
    }
    let mut len = 1;
    while len <= 3 && start + len <= s.len() {
        let seg = std::str::from_utf8(&s[start..start + len]).unwrap();
        if seg.len() > 1 && seg.as_bytes()[0] == b'0' {
            break;
        }
        if seg.parse::<u32>().unwrap() > 255 {
            break;
        }
        cur.push(seg.to_string());
        backtrack(s, start + len, part + 1, cur, res);
        cur.pop();
        len += 1;
    }
}

fn restore_ip(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut cur = Vec::new();
    backtrack(s.as_bytes(), 0, 0, &mut cur, &mut res);
    res
}

fn main() {
    let s = "2542540123";
    let res = restore_ip(s);
    let parts: Vec<String> = res.iter().map(|ip| format!("'{}'", ip)).collect();
    println!("[{}]", parts.join(", "));
}
