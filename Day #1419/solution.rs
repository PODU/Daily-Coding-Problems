// Day 1419: reverse the order of space-delimited words, in-place.
// Approach: reverse whole byte buffer, then reverse each word. O(n) time, O(1) extra space.

fn reverse_range(s: &mut [u8], mut i: usize, mut j: usize) {
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn reverse_words(s: &mut Vec<u8>) {
    let n = s.len();
    if n > 0 {
        reverse_range(s, 0, n - 1);
    }
    let mut start = 0;
    for i in 0..=n {
        if i == n || s[i] == b' ' {
            if i > start {
                reverse_range(s, start, i - 1);
            }
            start = i + 1;
        }
    }
}

fn main() {
    let mut s = "hello world here".to_string().into_bytes();
    reverse_words(&mut s);
    println!("{}", String::from_utf8(s).unwrap()); // here world hello
}
