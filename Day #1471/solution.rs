// Day 1471: O(log N) search in a sorted array with no *, /, or bit-shift.
// Build powers of two by doubling (addition); jump-based binary search.
// Time O(log N), Space O(log N).

fn search(a: &[i32], x: i32) -> bool {
    let n = a.len();
    if n == 0 {
        return false;
    }
    let mut powers = Vec::new();
    let mut p: usize = 1;
    while p <= n {
        powers.push(p);
        p = p + p;
    }
    let mut pos: i64 = -1;
    for &pw in powers.iter().rev() {
        let nxt = pos + pw as i64;
        if nxt < n as i64 && a[nxt as usize] <= x {
            pos = nxt;
        }
    }
    pos >= 0 && a[pos as usize] == x
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11];
    println!("{}", search(&arr, 7)); // true
    println!("{}", search(&arr, 8)); // false
}
