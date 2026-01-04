// Day 853: smallest distance (number of words between) between two words in a text.
// Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.
fn min_distance(text: &str, w1: &str, w2: &str) -> i32 {
    let mut p1: i32 = -1;
    let mut p2: i32 = -1;
    let mut best = i32::MAX;
    for (i, w) in text.split_whitespace().enumerate() {
        let i = i as i32;
        if w == w1 {
            p1 = i;
        }
        if w == w2 {
            p2 = i;
        }
        if p1 != -1 && p2 != -1 {
            best = best.min((p1 - p2).abs() - 1);
        }
    }
    best
}

fn main() {
    let text = "dog cat hello cat dog dog hello cat world";
    println!("{}", min_distance(text, "hello", "world")); // 1
}
