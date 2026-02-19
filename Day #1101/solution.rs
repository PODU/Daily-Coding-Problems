// Day 1101: Assign mice to holes minimizing the maximum travel distance.
// Sort both, match in order; answer is max |mice[i]-holes[i]|.
// Time: O(N log N). Space: O(1).
fn min_last_mouse(mut mice: Vec<i32>, mut holes: Vec<i32>) -> i32 {
    mice.sort();
    holes.sort();
    mice.iter().zip(holes.iter()).map(|(m, h)| (m - h).abs()).max().unwrap_or(0)
}

fn main() {
    println!("{}", min_last_mouse(vec![1, 4, 9, 15], vec![10, -5, 0, 16])); // 6
}
