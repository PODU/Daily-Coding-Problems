// Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.
fn can_reach(a: &[i32]) -> bool {
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
    println!("{}", can_reach(&[2, 0, 1, 0])); // true
    println!("{}", can_reach(&[1, 1, 0, 1])); // false
}
