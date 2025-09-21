// Day 311: Find a peak (greater than both neighbors) when ends are the lowest.
// Binary search toward the rising side. O(log N).
fn find_peak(a: &[i32]) -> usize {
    let (mut lo, mut hi) = (0usize, a.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < a[mid + 1] { lo = mid + 1; } else { hi = mid; }
    }
    lo
}

fn main() {
    let a = [1, 3, 5, 7, 6, 4, 2];
    let p = find_peak(&a);
    println!("index {} value {}", p, a[p]); // index 3 value 7
}
