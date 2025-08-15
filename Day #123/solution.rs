// Day 123: Validate whether a string is a number (int/real/scientific).
// Single linear scan state machine. O(n) time, O(1) space.
fn is_number(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    let mut i = 0;
    if n == 0 {
        return false;
    }
    if b[i] == b'+' || b[i] == b'-' {
        i += 1;
    }
    let (mut digits, mut dots) = (0, 0);
    while i < n && (b[i].is_ascii_digit() || b[i] == b'.') {
        if b[i] == b'.' {
            dots += 1;
        } else {
            digits += 1;
        }
        i += 1;
    }
    if dots > 1 || digits == 0 {
        return false;
    }
    if i < n && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < n && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let mut expd = 0;
        while i < n && b[i].is_ascii_digit() {
            expd += 1;
            i += 1;
        }
        if expd == 0 {
            return false;
        }
    }
    i == n
}

fn main() {
    let tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
    for t in tests.iter() {
        println!("\"{}\" -> {}", t, is_number(t));
    }
}
