// Next permutation of digits: find pivot, swap with next-larger suffix digit,
// reverse suffix. Time: O(d) digits, Space: O(d).

fn next_permutation(num: u64) -> u64 {
    let mut d: Vec<u8> = num.to_string().into_bytes();
    let n = d.len();
    if n < 2 {
        return num;
    }
    let mut i = n as isize - 2;
    while i >= 0 && d[i as usize] >= d[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return num; // already largest permutation
    }
    let i = i as usize;
    let mut j = n - 1;
    while d[j] <= d[i] {
        j -= 1;
    }
    d.swap(i, j);
    d[i + 1..].reverse();
    String::from_utf8(d).unwrap().parse().unwrap()
}

fn main() {
    println!("{}", next_permutation(48975));
}
