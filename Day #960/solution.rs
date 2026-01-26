// Day 960: jump game - can we reach the last index? Greedy furthest-reach.
// Time O(n), Space O(1).

fn can_reach(a: &[usize]) -> bool {
    let mut reach = 0usize;
    for (i, &step) in a.iter().enumerate() {
        if i > reach {
            return false;
        }
        reach = reach.max(i + step);
    }
    true
}

fn main() {
    println!("{}", can_reach(&[1, 3, 1, 2, 0, 1])); // true
    println!("{}", can_reach(&[1, 2, 1, 0, 0]));    // false
}
