// Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
fn main() {
    let nums = vec![1, 2, 3];
    let n = nums.len();
    let mut subsets: Vec<Vec<i32>> = Vec::new();
    for mask in 0..(1u32 << n) {
        let mut s = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                s.push(nums[i]);
            }
        }
        subsets.push(s);
    }
    subsets.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    let parts: Vec<String> = subsets
        .iter()
        .map(|s| {
            let e: Vec<String> = s.iter().map(|v| v.to_string()).collect();
            format!("[{}]", e.join(", "))
        })
        .collect();
    println!("[{}]", parts.join(", "));
}
