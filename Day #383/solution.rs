// Mark covered indices for every substring occurrence, then wrap maximal runs.
// Time: O(|s| * sum|sub|), Space: O(|s|).

fn embolden(s: &str, lst: &[&str]) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut bold = vec![false; n];
    for sub in lst {
        if sub.is_empty() {
            continue;
        }
        let sub_chars: Vec<char> = sub.chars().collect();
        let m = sub_chars.len();
        if m > n {
            continue;
        }
        for start in 0..=(n - m) {
            if chars[start..start + m] == sub_chars[..] {
                for b in bold.iter_mut().skip(start).take(m) {
                    *b = true;
                }
            }
        }
    }
    let mut out = String::new();
    for i in 0..n {
        if bold[i] && (i == 0 || !bold[i - 1]) {
            out.push_str("<b>");
        }
        out.push(chars[i]);
        if bold[i] && (i == n - 1 || !bold[i + 1]) {
            out.push_str("</b>");
        }
    }
    out
}

fn main() {
    println!("{}", embolden("abcdefg", &["bc", "ef"]));
    println!("{}", embolden("abcdefg", &["bcd", "def"]));
}
