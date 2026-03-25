// Day 1266: Arrange numbers to form the largest integer.
// Sort by custom comparator a+b vs b+a (descending). O(n log n) comparisons.
fn largest_number(nums: &[i64]) -> String {
    let mut s: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    s.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab)
    });
    if s.is_empty() || s[0] == "0" {
        return "0".to_string();
    }
    s.concat()
}

fn main() {
    println!("{}", largest_number(&[10, 7, 76, 415]));
}
