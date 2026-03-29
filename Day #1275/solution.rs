// Day 1275: Longest palindromic substring via Manacher's algorithm. O(n) time, O(n) space.
fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let orig: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = vec!['^'];
    for &c in &orig {
        t.push('#');
        t.push(c);
    }
    t.push('#');
    t.push('$');
    let n = t.len();
    let mut p = vec![0i32; n];
    let (mut center, mut right) = (0i32, 0i32);
    for i in 1..(n - 1) {
        let ii = i as i32;
        if ii < right {
            p[i] = (right - ii).min(p[(2 * center - ii) as usize]);
        }
        while t[(ii + p[i] + 1) as usize] == t[(ii - p[i] - 1) as usize] {
            p[i] += 1;
        }
        if ii + p[i] > right {
            center = ii;
            right = ii + p[i];
        }
    }
    let (mut max_len, mut center_index) = (0i32, 0usize);
    for i in 1..(n - 1) {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }
    let start = (center_index as i32 - max_len) / 2;
    orig[start as usize..(start + max_len) as usize].iter().collect()
}

fn main() {
    println!("{:?}", longest_palindrome("aabcdcb")); // "bcdcb"
    println!("{:?}", longest_palindrome("bananas")); // "anana"
}
