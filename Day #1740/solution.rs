// Approach: deterministic single linear scan validating sign/digits/dot/exponent.
// Time O(n), Space O(1).
fn is_number(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return false;
    }
    let mut i = 0;
    if b[i] == b'+' || b[i] == b'-' {
        i += 1;
    }
    let mut digits = false;
    let mut dot = false;
    while i < n && (b[i].is_ascii_digit() || b[i] == b'.') {
        if b[i] == b'.' {
            if dot {
                return false;
            }
            dot = true;
        } else {
            digits = true;
        }
        i += 1;
    }
    if !digits {
        return false;
    }
    if i < n && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < n && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let mut exp_digits = false;
        while i < n && b[i].is_ascii_digit() {
            exp_digits = true;
            i += 1;
        }
        if !exp_digits {
            return false;
        }
    }
    i == n
}

fn main() {
    let tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
    for t in tests.iter() {
        println!("{}", if is_number(t) { "true" } else { "false" });
    }
}
