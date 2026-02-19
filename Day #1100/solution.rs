// Day 1100: Search sorted array in O(log N) using only addition/comparison
// (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
// Time: O(log N). Space: O(log N).
fn contains(a: &[i32], x: i32) -> bool {
    let n = a.len() as i32;
    if n == 0 {
        return false;
    }
    let mut pows = Vec::new();
    let mut p = 1i32;
    while p <= n {
        pows.push(p);
        p += p; // doubling via addition
    }
    let mut pos: i32 = -1;
    for &p in pows.iter().rev() {
        if pos + p < n && a[(pos + p) as usize] <= x {
            pos += p;
        }
    }
    pos >= 0 && a[pos as usize] == x
}

fn main() {
    let a = [1, 3, 5, 7, 9, 11];
    println!("{}", contains(&a, 7)); // true
    println!("{}", contains(&a, 8)); // false
}
