// Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).
fn main() {
    let mut mice = vec![1, 4, 9, 15];
    let mut holes = vec![10, -5, 0, 16];
    mice.sort();
    holes.sort();
    let ans = mice
        .iter()
        .zip(holes.iter())
        .map(|(m, h)| (m - h).abs())
        .max()
        .unwrap_or(0);
    println!("{}", ans);
}
