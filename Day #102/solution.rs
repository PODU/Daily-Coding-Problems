// Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
// prefix p look for p-K seen earlier; earliest-ending match. O(n) time.
use std::collections::HashMap;

fn subarray_sum(nums: &[i64], k: i64) -> Option<Vec<i64>> {
    let mut first: HashMap<i64, i64> = HashMap::new();
    first.insert(0, -1);
    let mut prefix = 0i64;
    for (j, &x) in nums.iter().enumerate() {
        prefix += x;
        if let Some(&i) = first.get(&(prefix - k)) {
            return Some(nums[(i + 1) as usize..=j].to_vec());
        }
        first.entry(prefix).or_insert(j as i64);
    }
    None
}

fn main() {
    println!("{:?}", subarray_sum(&[1, 2, 3, 4, 5], 9)); // Some([2, 3, 4])
}
