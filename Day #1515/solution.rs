// Sort number-strings by comparator a+b > b+a (largest concat first), join; handle all-zeros.
// Time: O(n log n * L) comparisons, Space: O(n).
fn largest_number(nums: &[i32]) -> String {
    let mut s: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    s.sort_by(|a, b| {
        let ab = format!("{}{}", a, b);
        let ba = format!("{}{}", b, a);
        ba.cmp(&ab)
    });
    if s[0] == "0" {
        return "0".to_string();
    }
    s.concat()
}

fn main() {
    println!("{}", largest_number(&[10, 7, 76, 415]));
}
