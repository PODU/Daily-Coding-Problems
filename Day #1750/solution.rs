// Day 1750: Jump game — can advance at most arr[i] steps from index i.
// Greedy: track farthest reachable index in one pass.
// Time O(n), Space O(1).

fn can_reach(a: &[i32]) -> bool {
    let mut reach: i32 = 0;
    for (i, &step) in a.iter().enumerate() {
        if i as i32 > reach {
            return false;
        }
        reach = reach.max(i as i32 + step);
    }
    true
}

fn main() {
    println!("{}", can_reach(&[1, 3, 1, 2, 0, 1])); // true
    println!("{}", can_reach(&[1, 2, 1, 0, 0]));    // false
}
