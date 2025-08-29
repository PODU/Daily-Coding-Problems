// Day 192: Jump game -- can we reach the end (each value caps the jump length).
// Greedy farthest-reach. Time O(n), Space O(1).
fn can_reach_end(a: &[usize]) -> bool {
    let mut reach = 0usize;
    for (i, &x) in a.iter().enumerate() {
        if i > reach {
            return false;
        }
        reach = reach.max(i + x);
    }
    true
}

fn main() {
    println!("{}", can_reach_end(&[1, 3, 1, 2, 0, 1]));
    println!("{}", can_reach_end(&[1, 2, 1, 0, 0]));
}
