// Reverse word order in-place: reverse whole char array, then reverse each word.
// Time: O(n), Space: O(1) extra (on the mutable byte buffer).

fn reverse_range(a: &mut [u8], mut i: usize, mut j: usize) {
    while i < j {
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn reverse_words(a: &mut Vec<u8>) {
    let n = a.len();
    if n > 0 {
        reverse_range(a, 0, n - 1);
    }
    let mut start = 0;
    for i in 0..=n {
        if i == n || a[i] == b' ' {
            if i > start {
                reverse_range(a, start, i - 1);
            }
            start = i + 1;
        }
    }
}

fn main() {
    let mut a: Vec<u8> = "hello world here".bytes().collect();
    reverse_words(&mut a);
    println!("\"{}\"", String::from_utf8(a).unwrap());
}
