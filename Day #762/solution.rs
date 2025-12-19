// Day 762: Smallest distance (words in between) between two target words.
// Single pass tracking last seen index of each word. Time: O(n), Space: O(1).
fn smallest_distance(words: &[&str], a: &str, b: &str) -> i32 {
    let mut last_a: i32 = -1;
    let mut last_b: i32 = -1;
    let mut best_gap = i32::MAX;
    for (i, &w) in words.iter().enumerate() {
        let i = i as i32;
        if w == a {
            last_a = i;
            if last_b != -1 {
                best_gap = best_gap.min(last_a - last_b);
            }
        }
        if w == b {
            last_b = i;
            if last_a != -1 {
                best_gap = best_gap.min(last_b - last_a);
            }
        }
    }
    if best_gap == i32::MAX {
        -1
    } else {
        best_gap - 1
    }
}

fn main() {
    let text = "dog cat hello cat dog dog hello cat world";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{}", smallest_distance(&words, "hello", "world")); // 1
}
