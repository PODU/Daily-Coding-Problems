// Day 1148: Rotate linked list right by k.
// Vec-based rotation (clear ownership): rotate_right by k%len. O(n) time, O(n) space.
fn rotate(vals: &[i32], k: usize) -> Vec<i32> {
    let n = vals.len();
    if n == 0 {
        return vec![];
    }
    let k = k % n;
    let mut v = vals.to_vec();
    v.rotate_right(k);
    v
}

fn main() {
    let a = rotate(&[7, 7, 3, 5], 2);
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" -> ")); // 3 -> 5 -> 7 -> 7
    let b = rotate(&[1, 2, 3, 4, 5], 3);
    println!("{}", b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" -> ")); // 3 -> 4 -> 5 -> 1 -> 2
}
