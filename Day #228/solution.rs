// Largest number: sort by comparator a+b > b+a (concatenation), then join.
// Time: O(n log n * L), Space: O(n * L).
fn largest_number(nums: &[i64]) -> String {
    let mut s: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    s.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
    if s[0] == "0" {
        return "0".to_string();
    }
    s.concat()
}

fn main() {
    println!("{}", largest_number(&[10, 7, 76, 415])); // 77641510
}
