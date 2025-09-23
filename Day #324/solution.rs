// Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
// Time: O(N log N), Space: O(1) extra.
fn min_max_distance(mut mice: Vec<i32>, mut holes: Vec<i32>) -> i32 {
    mice.sort();
    holes.sort();
    mice.iter()
        .zip(holes.iter())
        .map(|(m, h)| (m - h).abs())
        .max()
        .unwrap_or(0)
}

fn main() {
    let mice = vec![1, 4, 9, 15];
    let holes = vec![10, -5, 0, 16];
    println!("{}", min_max_distance(mice, holes));
}
