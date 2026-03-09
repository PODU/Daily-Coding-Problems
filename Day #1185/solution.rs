// Day 1185: Smallest distance (words in between) between two words in a text.
// Single pass tracking last index of each target word; min |i-j| - 1.
// Time O(N), Space O(1).

fn shortest_distance(text: &str, w1: &str, w2: &str) -> i32 {
    let (mut p1, mut p2) = (-1i32, -1i32);
    let mut best = i32::MAX;
    for (i, token) in text.split_whitespace().enumerate() {
        let i = i as i32;
        if token == w1 {
            p1 = i;
        }
        if token == w2 {
            p2 = i;
        }
        if p1 >= 0 && p2 >= 0 {
            best = best.min((p1 - p2).abs());
        }
    }
    if best == i32::MAX {
        -1
    } else {
        best - 1
    }
}

fn main() {
    let text = "dog cat hello cat dog dog hello cat world";
    println!("{}", shortest_distance(text, "hello", "world")); // 1
}
