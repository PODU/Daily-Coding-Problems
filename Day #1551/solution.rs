// Sentence validator over a char stream: split on terminal marks, validate each candidate.
// isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).

fn is_terminal(c: char) -> bool { c == '.' || c == '?' || c == '!' || c == '\u{203D}' }
fn is_separator(c: char) -> bool { c == ',' || c == ';' || c == ':' }
fn is_lower(c: char) -> bool { c >= 'a' && c <= 'z' }
fn is_upper(c: char) -> bool { c >= 'A' && c <= 'Z' }

fn is_valid(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    let n = v.len();
    if n < 2 {
        return false;
    }
    if !is_upper(v[0]) {
        return false;
    }
    if !(is_lower(v[1]) || v[1] == ' ') {
        return false;
    }
    if !is_terminal(v[n - 1]) {
        return false;
    }
    if !(is_lower(v[n - 2]) || is_upper(v[n - 2])) {
        return false;
    }
    for i in 1..n - 1 {
        let c = v[i];
        if is_lower(c) || is_separator(c) {
            continue;
        }
        if c == ' ' {
            if v[i - 1] == ' ' {
                return false;
            }
            continue;
        }
        return false;
    }
    true
}

fn main() {
    let stream = "Hello world. this is bad.";
    let mut buf = String::new();
    for ch in stream.chars() {
        buf.push(ch);
        if is_terminal(ch) {
            let candidate = buf.trim().to_string();
            if is_valid(&candidate) {
                println!("{}", candidate);
            }
            buf.clear();
        }
    }
}
