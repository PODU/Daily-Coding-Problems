// Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
// Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
use std::collections::{HashMap, VecDeque};

fn main() {
    let preferences: Vec<(i32, Vec<i32>)> = vec![
        (0, vec![0, 1, 3, 6]),
        (1, vec![1, 4, 7]),
        (2, vec![2, 4, 7, 5]),
        (3, vec![3, 2, 5]),
        (4, vec![5, 8]),
    ];

    let mut cust_idx: HashMap<i32, usize> = HashMap::new();
    for (i, (c, _)) in preferences.iter().enumerate() {
        cust_idx.insert(*c, i);
    }
    let m = preferences.len();
    let full: u32 = (1u32 << m) - 1;

    let mut drink_mask: HashMap<i32, u32> = HashMap::new();
    for (c, drinks) in &preferences {
        for d in drinks {
            *drink_mask.entry(*d).or_insert(0) |= 1u32 << cust_idx[c];
        }
    }

    const INF: i32 = 1 << 30;
    let mut dp = vec![INF; (full + 1) as usize];
    dp[0] = 0;
    let mut q: VecDeque<u32> = VecDeque::new();
    q.push_back(0);
    while let Some(mask) = q.pop_front() {
        if mask == full {
            continue;
        }
        for dm in drink_mask.values() {
            let nm = mask | dm;
            if dp[nm as usize] > dp[mask as usize] + 1 {
                dp[nm as usize] = dp[mask as usize] + 1;
                q.push_back(nm);
            }
        }
    }
    println!("{}", dp[full as usize]);
}
