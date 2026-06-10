// Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
// Time: O(log N), Space: O(1).

fn midpoint(lo: i64, hi: i64) -> i64 {
    let mut i = lo;
    let mut j = hi;
    while i < j {
        i += 1;
        j -= 1;
    }
    j // floor((lo+hi)/2) using only +/-
}

fn contains(arr: &[i64], x: i64) -> bool {
    let mut lo: i64 = 0;
    let mut hi: i64 = arr.len() as i64 - 1;
    while lo <= hi {
        let mid = midpoint(lo, hi);
        let v = arr[mid as usize];
        if v == x {
            return true;
        } else if v < x {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    false
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];
    println!("{}", if contains(&arr, 7) { "true" } else { "false" });
    println!("{}", if contains(&arr, 8) { "true" } else { "false" });
}
