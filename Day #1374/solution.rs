// Jump game: greedy track furthest reachable index. Time O(n), Space O(1).
fn can_reach(a: &[i32]) -> bool {
    let mut reach = 0i32;
    for (i, &v) in a.iter().enumerate() {
        if i as i32 > reach {
            return false;
        }
        reach = reach.max(i as i32 + v);
    }
    true
}

fn main() {
    println!("{}", if can_reach(&[2, 0, 1, 0]) { "True" } else { "False" });
    println!("{}", if can_reach(&[1, 1, 0, 1]) { "True" } else { "False" });
}
