// Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
fn can_jump(a: &[i64]) -> bool {
    let mut reach: i64 = 0;
    for (i, &v) in a.iter().enumerate() {
        if i as i64 > reach {
            return false;
        }
        reach = reach.max(i as i64 + v);
    }
    true
}

fn main() {
    println!("{}", can_jump(&[1, 3, 1, 2, 0, 1]));
    println!("{}", can_jump(&[1, 2, 1, 0, 0]));
}
