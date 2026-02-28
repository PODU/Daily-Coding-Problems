// Phone keypad letter combinations via backtracking. O(prod of choices) time.

fn letters(d: char) -> &'static str {
    match d {
        '2' => "abc", '3' => "def", '4' => "ghi", '5' => "jkl",
        '6' => "mno", '7' => "pqrs", '8' => "tuv", '9' => "wxyz",
        _ => "",
    }
}

fn backtrack(digits: &[char], i: usize, cur: &mut String, out: &mut Vec<String>) {
    if i == digits.len() {
        out.push(cur.clone());
        return;
    }
    for c in letters(digits[i]).chars() {
        cur.push(c);
        backtrack(digits, i + 1, cur, out);
        cur.pop();
    }
}

fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let chars: Vec<char> = digits.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    backtrack(&chars, 0, &mut cur, &mut out);
    out
}

fn main() {
    let res = letter_combinations("23");
    println!("[{}]", res.join(", "));
}
