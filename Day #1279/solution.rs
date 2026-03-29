// Day 1279: Lazy bartender = minimum set cover over customers.
// DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
use std::collections::HashMap;

fn min_drinks(prefs: &[Vec<i32>]) -> i32 {
    let c = prefs.len();
    let mut drink_mask: HashMap<i32, u32> = HashMap::new();
    for (cust, list) in prefs.iter().enumerate() {
        for &d in list {
            *drink_mask.entry(d).or_insert(0) |= 1u32 << cust;
        }
    }
    let full = (1u32 << c) - 1;
    let inf = i32::MAX;
    let mut dp = vec![inf; 1usize << c];
    dp[0] = 0;
    let masks: Vec<u32> = drink_mask.values().cloned().collect();
    for mask in 0..=full {
        if dp[mask as usize] == inf {
            continue;
        }
        for &dm in &masks {
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
        vec![0, 1, 3, 6],
        vec![1, 4, 7],
        vec![2, 4, 7, 5],
        vec![3, 2, 5],
        vec![5, 8],
    ];
    println!("{}", min_drinks(&prefs)); // 2
}
