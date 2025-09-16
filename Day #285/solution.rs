// Day 285: Count buildings (east->west) with a sunset (west) view.
// Single backward pass tracking running max. Time O(N), Space O(1).
fn sunset_views(h: &[i32]) -> usize {
    let mut count = 0;
    let mut max_so_far = i32::MIN;
    for &x in h.iter().rev() {
        if x > max_so_far {
            count += 1;
            max_so_far = x;
        }
    }
    count
}

fn main() {
    println!("{}", sunset_views(&[3, 7, 8, 3, 6, 1])); // 3
}
