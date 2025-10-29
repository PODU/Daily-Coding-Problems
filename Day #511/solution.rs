// Day 511: Minimum jumps to reach end of array (each value = max jump length).
// Greedy level-by-level reachability. Time O(n), Space O(1).
fn min_jumps(a: &[i32]) -> i32 {
    let n = a.len();
    if n <= 1 {
        return 0;
    }
    let (mut jumps, mut cur_end, mut farthest) = (0, 0usize, 0usize);
    for i in 0..n - 1 {
        farthest = farthest.max(i + a[i] as usize);
        if i == cur_end {
            jumps += 1;
            cur_end = farthest;
            if cur_end >= n - 1 {
                break;
            }
        }
    }
    jumps
}

fn main() {
    let a = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9];
    println!("{}", min_jumps(&a));
}
