// Roman to decimal: add each value, subtract when a smaller numeral precedes a larger. Time O(n), Space O(1).
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
        if i + 1 < chars.len() && value(chars[i]) < value(chars[i + 1]) {
            total -= value(chars[i]);
        } else {
            total += value(chars[i]);
        }
    }
    total
}

fn main() {
    println!("{}", roman_to_int("XIV"));
}
