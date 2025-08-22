// Day 153: Min words separating two words. Single pass tracking last seen index
// of each word; answer is min(|i-j|-1). Time O(n), Space O(1).

fn min_distance(words: &[&str], a: &str, b: &str) -> i32 {
    let mut last_a: i32 = -1;
    let mut last_b: i32 = -1;
    let mut best = i32::MAX;
    for (i, &w) in words.iter().enumerate() {
        let i = i as i32;
        if w == a {
            last_a = i;
            if last_b != -1 {
                best = best.min((last_a - last_b).abs() - 1);
            }
        }
        if w == b {
            last_b = i;
            if last_a != -1 {
                best = best.min((last_a - last_b).abs() - 1);
            }
        }
    }
    best
}

fn main() {
    let text = "dog cat hello cat dog dog hello cat world";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{}", min_distance(&words, "hello", "world"));
}
