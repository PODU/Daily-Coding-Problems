// Day 1321: Roman numeral -> decimal.
// Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.

fn value(c: char) -> i32 {
    match c {
        'M' => 1000, 'D' => 500, 'C' => 100, 'L' => 50,
        'X' => 10, 'V' => 5, 'I' => 1, _ => 0,
    }
}

fn roman_to_int(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut total = 0;
    for i in 0..chars.len() {
        let cur = value(chars[i]);
        if i + 1 < chars.len() && cur < value(chars[i + 1]) {
            total -= cur;
        } else {
            total += cur;
        }
    }
    total
}

fn main() {
    println!("{}", roman_to_int("XIV")); // 14
}
