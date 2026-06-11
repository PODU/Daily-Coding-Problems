// Next permutation of the digit array: find rightmost ascending pair, swap with
// next-greater suffix digit, reverse suffix. Time O(d), Space O(d).

fn next_permutation(num: i64) -> i64 {
    let mut s: Vec<u8> = num.to_string().into_bytes();
    let n = s.len();
    if n < 2 {
        return -1;
    }
    let mut i = n as i64 - 2;
    while i >= 0 && s[i as usize] >= s[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return -1; // no next permutation
    }
    let i = i as usize;
    let mut j = n - 1;
    while s[j] <= s[i] {
        j -= 1;
    }
    s.swap(i, j);
    s[i + 1..].reverse();
    String::from_utf8(s).unwrap().parse().unwrap()
}

fn main() {
    println!("{}", next_permutation(48975));
}
