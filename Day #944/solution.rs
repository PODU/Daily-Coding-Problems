// Day 944: Next greater permutation of an integer's digits (in-place on digit array).
// Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).

// Returns -1 if no greater permutation exists.
fn next_permutation(num: i64) -> i64 {
    let mut d: Vec<u8> = num.to_string().into_bytes();
    let n = d.len();
    if n < 2 {
        return -1;
    }
    let mut i = n as isize - 2;
    while i >= 0 && d[i as usize] >= d[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return -1;
    }
    let i = i as usize;
    let mut j = n - 1;
    while d[j] <= d[i] {
        j -= 1;
    }
    d.swap(i, j);
    d[i + 1..].reverse();
    String::from_utf8(d).unwrap().parse::<i64>().unwrap()
}

fn main() {
    println!("{}", next_permutation(48975)); // 49578
}
