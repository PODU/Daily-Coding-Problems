// Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
fn can_jump(nums: &[i32]) -> bool {
    let mut farthest: i32 = 0;
    for (i, &v) in nums.iter().enumerate() {
        if i as i32 > farthest {
            return false;
        }
        farthest = farthest.max(i as i32 + v);
    }
    true
}

fn main() {
    println!("{}", if can_jump(&[2, 0, 1, 0]) { "True" } else { "False" });
    println!("{}", if can_jump(&[1, 1, 0, 1]) { "True" } else { "False" });
}
