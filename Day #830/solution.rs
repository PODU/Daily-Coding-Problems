// Day 830: Largest number formed by concatenating the given numbers.
// Sort strings by comparator a+b vs b+a (descending). O(N log N) compares of O(L) strings.

fn largest_number(nums: &[i64]) -> String {
    let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    strs.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab) // descending: b+a vs a+b
    });
    let result: String = strs.concat();
    if result.starts_with('0') {
        return "0".to_string();
    }
    result
}

fn main() {
    println!("{}", largest_number(&[10, 7, 76, 415])); // 77641510
}
