// Day 1300: Find a contiguous subarray summing to K (handles negatives).
// Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.
use std::collections::HashMap;

fn subarray_sum(a: &[i64], k: i64) -> Vec<i64> {
    let mut first_index: HashMap<i64, i64> = HashMap::new();
    first_index.insert(0, -1);
    let mut prefix = 0i64;
    for (j, &v) in a.iter().enumerate() {
        prefix += v;
        if let Some(&i) = first_index.get(&(prefix - k)) {
            return a[(i + 1) as usize..=j].to_vec();
        }
        first_index.entry(prefix).or_insert(j as i64);
    }
    vec![]
}

fn main() {
    println!("{:?}", subarray_sum(&[1, 2, 3, 4, 5], 9)); // [2, 3, 4]
}
