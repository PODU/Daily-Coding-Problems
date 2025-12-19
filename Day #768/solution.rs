// Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
// Process elements in pairs: compare the pair, then smaller vs min, larger vs max.
fn min_max(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let (mut mn, mut mx, mut i);
    if n % 2 == 0 {
        mn = a[0].min(a[1]);
        mx = a[0].max(a[1]);
        i = 2;
    } else {
        mn = a[0];
        mx = a[0];
        i = 1;
    }
    while i < n {
        let (mut lo, mut hi) = (a[i], a[i + 1]);
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }
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
    let (mn, mx) = min_max(&[3, 5, 1, 2, 4, 8, 7]);
    println!("min={} max={}", mn, mx); // min=1 max=8
}
