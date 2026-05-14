// Reverse words in a space-delimited string.
// Approach: reverse whole byte vec, then reverse each word in place (in-place).
// Time: O(n), Space: O(1) extra.

fn reverse_range(s: &mut [u8], mut i: usize, mut j: usize) {
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn reverse_words(input: &str) -> String {
    let mut s = input.as_bytes().to_vec();
    let n = s.len();
    if n > 0 {
        reverse_range(&mut s, 0, n - 1);
    }
    let mut start = 0usize;
    for i in 0..=n {
        if i == n || s[i] == b' ' {
            if i > start {
                reverse_range(&mut s, start, i - 1);
            }
            start = i + 1;
        }
    }
    String::from_utf8(s).unwrap()
}

fn main() {
    println!("\"{}\"", reverse_words("hello world here")); // "here world hello"
}
