// Longest palindromic substring via Manacher's algorithm.
// Transform with '#' separators, expand radii using mirror symmetry.
// Time: O(n), Space: O(n). (ASCII input assumed for byte indexing.)

fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut t: Vec<char> = vec!['#'];
    for ch in s.chars() {
        t.push(ch);
        t.push('#');
    }
    let n = t.len();
    let mut p = vec![0i32; n];
    let (mut c, mut r) = (0i32, 0i32);
    for i in 0..n {
        let ii = i as i32;
        if ii < r {
            p[i] = (r - ii).min(p[(2 * c - ii) as usize]);
        }
        while ii - p[i] - 1 >= 0
            && ii + p[i] + 1 < n as i32
            && t[(ii - p[i] - 1) as usize] == t[(ii + p[i] + 1) as usize]
        {
            p[i] += 1;
        }
        if ii + p[i] > r {
            c = ii;
            r = ii + p[i];
        }
    }
    let mut max_len = 0i32;
    let mut center = 0usize;
    for i in 0..n {
        if p[i] > max_len {
            max_len = p[i];
            center = i;
        }
    }
    let start = (center as i32 - max_len) / 2;
    let chars: Vec<char> = s.chars().collect();
    chars[start as usize..(start + max_len) as usize].iter().collect()
}

fn main() {
    println!("{}", longest_palindrome("aabcdcb")); // bcdcb
    println!("{}", longest_palindrome("bananas")); // anana
}
