// Day 1125 - Contiguous sublist summing to K
// Prefix sums + hash map (handles negatives) to find a range with sum == K in
// one pass. Time: O(n), Space: O(n).
use std::collections::HashMap;

fn subarray_sum(nums: &[i64], k: i64) -> Option<Vec<i64>> {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    seen.insert(0, -1);
    let mut total = 0i64;
    for (j, &x) in nums.iter().enumerate() {
        total += x;
        if let Some(&i) = seen.get(&(total - k)) {
            return Some(nums[(i + 1) as usize..=j].to_vec());
        }
        seen.entry(total).or_insert(j as i64);
    }
    None
}

fn main() {
    let nums = vec![1i64, 2, 3, 4, 5];
    println!("{:?}", subarray_sum(&nums, 9).unwrap()); // [2, 3, 4]
}
