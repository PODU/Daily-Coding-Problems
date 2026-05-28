// Day 1578: Find min and max using ~3N/2 comparisons (pairwise method).
// Compare elements in pairs, then each pair contributes one compare to min and one to max.
// Time: O(N) with ceil(3N/2)-2 comparisons; Space: O(1).

fn min_max(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let (mut mn, mut mx, mut i);
    if n & 1 == 1 {
        mn = a[0];
        mx = a[0];
        i = 1;
    } else {
        if a[0] < a[1] {
            mn = a[0];
            mx = a[1];
        } else {
            mn = a[1];
            mx = a[0];
        }
        i = 2;
    }
    while i + 1 < n {
        let (lo, hi) = if a[i] < a[i + 1] { (a[i], a[i + 1]) } else { (a[i + 1], a[i]) };
        if lo < mn {
            mn = lo;
        }
        if hi > mx {
            mx = hi;
        }
        i += 2;
    }
    (mn, mx)
}

fn main() {
    let (mn, mx) = min_max(&[3, 5, 1, 2, 8, 7, 4]);
    println!("min={} max={}", mn, mx); // min=1 max=8
}
