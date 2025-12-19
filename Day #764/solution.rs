// Day 764: Arrange numbers to form the largest integer.
// Sort by comparator: a+b vs b+a (descending). Time: O(n log n * L), Space: O(n).
fn largest_number(nums: &[i64]) -> String {
    let mut s: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    s.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab) // descending by concatenation
    });
    if s[0] == "0" {
        return "0".to_string();
    }
    s.concat()
}

fn main() {
    let nums = [10i64, 7, 76, 415];
    println!("{}", largest_number(&nums)); // 77641510
}
