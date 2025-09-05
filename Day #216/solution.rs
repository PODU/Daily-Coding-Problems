// Day 216: Roman numeral -> decimal.
// Approach: scan left-to-right; if current value < next, subtract, else add. Time O(n), Space O(1).
fn value(c: u8) -> i32 {
    match c {
        b'M' => 1000,
        b'D' => 500,
        b'C' => 100,
        b'L' => 50,
        b'X' => 10,
        b'V' => 5,
        b'I' => 1,
        _ => 0,
    }
}

fn roman_to_int(s: &str) -> i32 {
    let b = s.as_bytes();
    let mut total = 0;
    for i in 0..b.len() {
        if i + 1 < b.len() && value(b[i]) < value(b[i + 1]) {
            total -= value(b[i]);
        } else {
            total += value(b[i]);
        }
    }
    total
}

fn main() {
    println!("{}", roman_to_int("XIV")); // 14
}
