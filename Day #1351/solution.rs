// Pairwise min/max: process pairs, compare smaller->min, larger->max -> ~3N/2 comparisons.
// Time: O(N) (~3N/2 comparisons), Space: O(1).
fn min_max(a: &[i32]) -> (i32, i32) {
    let n = a.len();
    let (mut mn, mut mx, mut i);
    if n % 2 == 0 {
        if a[0] < a[1] { mn = a[0]; mx = a[1]; } else { mn = a[1]; mx = a[0]; }
        i = 2;
    } else {
        mn = a[0];
        mx = a[0];
        i = 1;
    }
    while i + 1 <= n {
        let (small, large) = if a[i] < a[i + 1] { (a[i], a[i + 1]) } else { (a[i + 1], a[i]) };
        if small < mn { mn = small; }
        if large > mx { mx = large; }
        i += 2;
    }
    (mn, mx)
}

fn main() {
    let a = [3, 5, 1, 2, 4, 8, 7];
    let (mn, mx) = min_max(&a);
    println!("Min: {}, Max: {}", mn, mx);
}
