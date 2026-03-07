// Day 1174: Decide whether a string is a valid number.
// Single linear scan: optional sign, integer/fraction digits, optional exponent.
// Time O(N), Space O(1).

fn is_number(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    let mut i = 0;
    if n == 0 {
        return false;
    }
    let digit = |k: usize| k < n && b[k].is_ascii_digit();
    if i < n && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    let mut before = false;
    let mut after = false;
    while digit(i) {
        i += 1;
        before = true;
    }
    if i < n && b[i] == b'.' {
        i += 1;
        while digit(i) {
            i += 1;
            after = true;
        }
    }
    if !before && !after {
        return false;
    }
    if i < n && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < n && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let mut exp = false;
        while digit(i) {
            i += 1;
            exp = true;
        }
        if !exp {
            return false;
        }
    }
    i == n
}

fn main() {
    let tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
    for t in tests.iter() {
        println!("\"{}\" -> {}", t, if is_number(t) { "true" } else { "false" });
    }
}
