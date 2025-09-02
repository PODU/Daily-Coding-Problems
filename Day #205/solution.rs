// Day 205: Next permutation of an integer's digits.
// Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
// Time: O(d), Space: O(d) for digits.
fn next_permutation(n: u64) -> u64 {
    let mut s: Vec<u8> = n.to_string().into_bytes();
    let len = s.len();
    if len < 2 {
        return n;
    }
    let mut i = len as isize - 2;
    while i >= 0 && s[i as usize] >= s[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return n; // already the largest permutation
    }
    let i = i as usize;
    let mut j = len - 1;
    while s[j] <= s[i] {
        j -= 1;
    }
    s.swap(i, j);
    s[i + 1..].reverse();
    String::from_utf8(s).unwrap().parse().unwrap()
}

fn main() {
    println!("{}", next_permutation(48975)); // 49578
}
