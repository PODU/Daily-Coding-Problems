// Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
// Time: O(N) with <2N comparisons. Space: O(1).

fn min_max(a: &[i32]) -> (i32, i32, u64) {
    let mut cmps: u64 = 0;
    let n = a.len();
    let (mut mn, mut mx);
    let mut i;
    if n % 2 == 1 {
        mn = a[0];
        mx = a[0];
        i = 1; // odd: seed with first
    } else {
        cmps += 1; // even: seed with first pair
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
        let (lo, hi);
        cmps += 1;
        if a[i] < a[i + 1] {
            lo = a[i];
            hi = a[i + 1];
        } else {
            lo = a[i + 1];
            hi = a[i];
        }
        cmps += 1;
        if lo < mn {
            mn = lo;
        }
        cmps += 1;
        if hi > mx {
            mx = hi;
        }
        i += 2;
    }
    (mn, mx, cmps)
}

fn main() {
    let a = [7, 2, 9, 4, 1, 8, 3];
    let (mn, mx, _) = min_max(&a);
    println!("min={} max={}", mn, mx);
}
