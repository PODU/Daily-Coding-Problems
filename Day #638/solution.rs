// Day 638: Reverse the words in a string (in-place on a byte buffer).
// Approach: reverse whole buffer, then reverse each word back.
// Time: O(n), Space: O(1) extra.
fn reverse_words(s: &str) -> String {
    let mut a: Vec<u8> = s.bytes().collect();
    let n = a.len();
    a.reverse();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && a[j] != b' ' {
            j += 1;
        }
        a[i..j].reverse();
        i = j + 1;
    }
    String::from_utf8(a).unwrap()
}

fn main() {
    println!("{}", reverse_words("hello world here")); // here world hello
}
