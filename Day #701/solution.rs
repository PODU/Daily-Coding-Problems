// Day 701: Minimum drinks to satisfy every customer (minimum set cover).
// Approach: each drink -> bitmask of customers it satisfies; DP over customer
// masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).
use std::collections::HashMap;

fn min_drinks(prefs: &[(i32, Vec<i32>)]) -> i32 {
    let c = prefs.len();
    let mut drink_mask: HashMap<i32, u32> = HashMap::new();
    for (i, (_, drinks)) in prefs.iter().enumerate() {
        for &d in drinks {
            *drink_mask.entry(d).or_insert(0) |= 1 << i;
        }
    }
    let full = (1u32 << c) - 1;
    let mut dp = vec![i32::MAX; (full + 1) as usize];
    dp[0] = 0;
    for mask in 0..=full {
        if dp[mask as usize] == i32::MAX {
            continue;
        }
        for &dm in drink_mask.values() {
            let nm = (mask | dm) as usize;
            if dp[mask as usize] + 1 < dp[nm] {
                dp[nm] = dp[mask as usize] + 1;
            }
        }
    }
    dp[full as usize]
}

fn main() {
    let prefs = vec![
        (0, vec![0, 1, 3, 6]),
        (1, vec![1, 4, 7]),
        (2, vec![2, 4, 7, 5]),
        (3, vec![3, 2, 5]),
        (4, vec![5, 8]),
    ];
    println!("{}", min_drinks(&prefs)); // 2
}
