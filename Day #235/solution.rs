// Min & Max in ~3N/2 comparisons: process elements in pairs, compare the pair first,
// then smaller vs min and larger vs max. Time: O(N), Space: O(1). Comparisons ~ 3*ceil(N/2)-2.
fn min_max(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let (mut mn, mut mx, mut i);
    if n % 2 == 0 {
        if a[0] < a[1] {
            mn = a[0];
            mx = a[1];
        } else {
            mn = a[1];
            mx = a[0];
        }
        i = 2;
    } else {
        mn = a[0];
        mx = a[0];
        i = 1;
    }
    while i < n {
        let (x, y) = (a[i], a[i + 1]);
        if x < y {
            if x < mn { mn = x; }
            if y > mx { mx = y; }
        } else {
            if y < mn { mn = y; }
            if x > mx { mx = x; }
        }
        i += 2;
    }
    (mn, mx)
}

fn main() {
    let a = [3, 5, 1, 2, 4, 8, 7];
    let (mn, mx) = min_max(&a);
    println!("min={} max={}", mn, mx); // min=1 max=8
}
