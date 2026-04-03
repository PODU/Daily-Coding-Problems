// Day 1291: Next permutation of an integer's digits.
// Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.
fn next_permutation(num: &str) -> String {
    let mut s: Vec<u8> = num.bytes().collect();
    let n = s.len();
    if n < 2 {
        return num.to_string();
    }
    let mut i = n as isize - 2;
    while i >= 0 && s[i as usize] >= s[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return num.to_string(); // already largest
    }
    let i = i as usize;
    let mut j = n - 1;
    while s[j] <= s[i] {
        j -= 1;
    }
    s.swap(i, j);
    s[i + 1..].reverse();
    String::from_utf8(s).unwrap()
}

fn main() {
    println!("{}", next_permutation("48975")); // 49578
}
