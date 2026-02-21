// Day 1111 - Lazy bartender (minimum set cover)
// Approach: exact set cover via DP over bitmask of covered customers.
// Time: O(D * 2^C), Space: O(2^C).
use std::collections::BTreeMap;
use std::collections::HashMap;

fn min_drinks(preferences: &BTreeMap<i32, Vec<i32>>) -> i32 {
    let customers: Vec<i32> = preferences.keys().cloned().collect();
    let n = customers.len();
    let mut cidx: HashMap<i32, usize> = HashMap::new();
    for (i, &c) in customers.iter().enumerate() {
        cidx.insert(c, i);
    }

    let mut drink_mask: HashMap<i32, u32> = HashMap::new();
    for (c, drinks) in preferences.iter() {
        for &d in drinks {
            *drink_mask.entry(d).or_insert(0) |= 1u32 << cidx[c];
        }
    }

    let full: usize = (1usize << n) - 1;
    let inf = i32::MAX;
    let mut dp = vec![inf; 1usize << n];
    dp[0] = 0;
    for s in 0..(1usize << n) {
        if dp[s] == inf {
            continue;
        }
        for &m in drink_mask.values() {
            let ns = s | (m as usize);
            if dp[ns] > dp[s] + 1 {
                dp[ns] = dp[s] + 1;
            }
        }
    }
    dp[full]
}

fn main() {
    let mut preferences: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    preferences.insert(0, vec![0, 1, 3, 6]);
    preferences.insert(1, vec![1, 4, 7]);
    preferences.insert(2, vec![2, 4, 7, 5]);
    preferences.insert(3, vec![3, 2, 5]);
    preferences.insert(4, vec![5, 8]);
    println!("{}", min_drinks(&preferences)); // 2
}
