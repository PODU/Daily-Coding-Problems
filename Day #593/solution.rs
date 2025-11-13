// Day 593: Count buildings with a view of the setting sun (west).
// Array is east->west (index 0 = east). A building sees the sunset iff it is
// taller than every building further west (higher index). Single right-to-left pass.
// Time: O(n), Space: O(1).
fn count_sunset_views(h: &[i32]) -> i32 {
    let mut count = 0;
    let mut max_seen = i32::MIN;
    for &x in h.iter().rev() {
        if x > max_seen {
            count += 1;
            max_seen = x;
        }
    }
    count
}

fn main() {
    println!("{}", count_sunset_views(&[3, 7, 8, 3, 6, 1])); // 3
}
