// Day 1177: Find the element appearing once while all others appear 3 times.
// Track bits seen once (ones) and twice (twos); a third sighting clears both.
// Time O(N), Space O(1).

fn single_number(a: &[i32]) -> i32 {
    let mut ones = 0i32;
    let mut twos = 0i32;
    for &x in a {
        ones = (ones ^ x) & !twos;
        twos = (twos ^ x) & !ones;
    }
    ones
}

fn main() {
    println!("{}", single_number(&[6, 1, 3, 3, 3, 6, 6])); // 1
    println!("{}", single_number(&[13, 19, 13, 13]));       // 19
}
