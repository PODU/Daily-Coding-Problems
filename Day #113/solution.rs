// Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
fn reverse_words(input: &str) -> String {
    let mut s: Vec<u8> = input.bytes().collect();
    let n = s.len();
    s.reverse();
    let mut start = 0;
    for i in 0..=n {
        if i == n || s[i] == b' ' {
            s[start..i].reverse();
            start = i + 1;
        }
    }
    String::from_utf8(s).unwrap()
}

fn main() {
    println!("\"{}\"", reverse_words("hello world here")); // "here world hello"
}
