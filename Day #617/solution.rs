// Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
// value precedes a larger one. Time O(n), Space O(1).
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
        let v = value(chars[i]);
        if i + 1 < chars.len() && value(chars[i + 1]) > v {
            total -= v;
        } else {
            total += v;
        }
    }
    total
}

fn main() {
    println!("{}", roman_to_int("XIV")); // 14
}
