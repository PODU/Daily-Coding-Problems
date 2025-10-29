// Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
use std::collections::HashMap;

fn subarray_sum(a: &[i64], k: i64) -> Vec<i64> {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    seen.insert(0, -1);
    let mut pre: i64 = 0;
    for (i, &v) in a.iter().enumerate() {
        pre += v;
        if let Some(&start) = seen.get(&(pre - k)) {
            return a[(start + 1) as usize..=i].to_vec();
        }
        seen.entry(pre).or_insert(i as i64);
    }
    Vec::new()
}

fn main() {
    println!("{:?}", subarray_sum(&[1, 2, 3, 4, 5], 9));
}
