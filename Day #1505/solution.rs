// Jump game: greedy, track furthest reachable index.
// Time O(n), Space O(1). Prints "True"/"False" (capitalized) per spec.

fn can_jump(a: &[i32]) -> bool {
    let mut reach: i32 = 0;
    for (i, &v) in a.iter().enumerate() {
        if i as i32 > reach {
            return false;
        }
        reach = reach.max(i as i32 + v);
    }
    true
}

fn main() {
    let a = vec![2, 0, 1, 0];
    println!("{}", if can_jump(&a) { "True" } else { "False" });
}
