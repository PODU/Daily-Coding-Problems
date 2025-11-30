// Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
// Time: O(n) over string length, Space: O(1).

fn is_valid_number(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    let mut i = 0;
    if n == 0 {
        return false;
    }
    if b[i] == b'+' || b[i] == b'-' {
        i += 1;
    }
    let (mut digits_before, mut digits_after) = (false, false);
    while i < n && b[i].is_ascii_digit() {
        i += 1;
        digits_before = true;
    }
    if i < n && b[i] == b'.' {
        i += 1;
        while i < n && b[i].is_ascii_digit() {
            i += 1;
            digits_after = true;
        }
    }
    if !digits_before && !digits_after {
        return false;
    }
    if i < n && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < n && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let mut exp_digits = false;
        while i < n && b[i].is_ascii_digit() {
            i += 1;
            exp_digits = true;
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
        let res = if is_valid_number(t) { "True" } else { "False" };
        println!("\"{}\" -> {}", t, res);
    }
}
