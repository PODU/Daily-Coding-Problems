// Single pass from the west end (array right), tracking the tallest seen so far;
// a building has a view iff it is taller than everything to its west.
// Time O(n), Space O(1).

fn count_sunset_views(h: &[i32]) -> i32 {
    let mut count = 0;
    let mut max_w = i32::MIN;
    for &x in h.iter().rev() {
        if x > max_w {
            count += 1;
            max_w = x;
        }
    }
    count
}

fn main() {
    println!("{}", count_sunset_views(&[3, 7, 8, 3, 6, 1])); // 3
}
