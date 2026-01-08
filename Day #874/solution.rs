// Longest palindromic substring via Manacher's algorithm. Time O(n), Space O(n).

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
    let mut center = 0i32;
    let mut right = 0i32;
    for i in 1..(n - 1) as i32 {
        let iu = i as usize;
        if i < right {
            p[iu] = (right - i).min(p[(2 * center - i) as usize]);
        }
        while t[(i + p[iu] + 1) as usize] == t[(i - p[iu] - 1) as usize] {
            p[iu] += 1;
        }
        if i + p[iu] > right {
            center = i;
            right = i + p[iu];
        }
    }
    let mut max_len = 0i32;
    let mut center_index = 0i32;
    for i in 1..(n - 1) {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i as i32;
        }
    }
    let start = ((center_index - max_len) / 2) as usize;
    orig[start..start + max_len as usize].iter().collect()
}

fn main() {
    println!("{:?}", longest_palindrome("aabcdcb"));
    println!("{:?}", longest_palindrome("bananas"));
}
