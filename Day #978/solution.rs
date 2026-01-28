// Valid number check via single-pass finite-state parser.
// Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
// Time: O(n); Space: O(1).

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

    let mut digits_before = false;
    let mut digits_after = false;
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
        return false; // mantissa needs a digit
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
            return false; // exponent needs a digit
        }
    }
    i == n // no trailing junk
}

fn main() {
    let valid = ["10", "-10", "10.1", "-10.1", "1e5"];
    let invalid = ["a", "x 1", "a -2", "-", "", " "];
    for s in valid.iter().chain(invalid.iter()) {
        println!("\"{}\" -> {}", s, is_number(s));
    }
}
