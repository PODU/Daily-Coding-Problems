// Min word distance: single pass tracking last-seen index of each word; on each
// hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).

fn min_word_distance(text: &[&str], w1: &str, w2: &str) -> i64 {
    let (mut last1, mut last2): (i64, i64) = (-1, -1);
    let mut best = i64::MAX;
    for (i, &word) in text.iter().enumerate() {
        let i = i as i64;
        if word == w1 {
            last1 = i;
            if last2 != -1 {
                best = best.min((last1 - last2).abs() - 1);
            }
        }
        if word == w2 {
            last2 = i;
            if last1 != -1 {
                best = best.min((last1 - last2).abs() - 1);
            }
        }
    }
    best
}

fn main() {
    let text: Vec<&str> = "dog cat hello cat dog dog hello cat world"
        .split_whitespace()
        .collect();
    println!("{}", min_word_distance(&text, "hello", "world"));
}
