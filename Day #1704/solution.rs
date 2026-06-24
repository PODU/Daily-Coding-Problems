// Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
// most uncovered customers. Time O(D^2*C), Space O(D*C).
use std::collections::{HashMap, HashSet};

fn main() {
    let preferences: Vec<(i32, Vec<i32>)> = vec![
        (0, vec![0, 1, 3, 6]),
        (1, vec![1, 4, 7]),
        (2, vec![2, 4, 7, 5]),
        (3, vec![3, 2, 5]),
        (4, vec![5, 8]),
    ];
    let mut drink_to_cust: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut uncovered: HashSet<i32> = HashSet::new();
    for (cust, drinks) in &preferences {
        uncovered.insert(*cust);
        for d in drinks {
            drink_to_cust.entry(*d).or_default().insert(*cust);
        }
    }
    let mut learned = 0;
    while !uncovered.is_empty() {
        let mut best_drink: Option<i32> = None;
        let mut best_count = 0;
        for (drink, custs) in &drink_to_cust {
            let cnt = custs.iter().filter(|c| uncovered.contains(c)).count();
            if cnt > best_count {
                best_count = cnt;
                best_drink = Some(*drink);
            }
        }
        match best_drink {
            None => break,
            Some(d) => {
                for c in &drink_to_cust[&d] {
                    uncovered.remove(c);
                }
                learned += 1;
            }
        }
    }
    println!("{}", learned);
}
