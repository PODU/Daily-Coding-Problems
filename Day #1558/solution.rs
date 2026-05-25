// Count connected components (friend groups) in an undirected graph via DFS.
// Time O(V+E), Space O(V).
use std::collections::{BTreeMap, HashSet};

fn main() {
    let mut adj: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    adj.insert(0, vec![1, 2]);
    adj.insert(1, vec![0, 5]);
    adj.insert(2, vec![0]);
    adj.insert(3, vec![6]);
    adj.insert(4, vec![]);
    adj.insert(5, vec![1]);
    adj.insert(6, vec![3]);

    let mut visited: HashSet<i32> = HashSet::new();
    let mut groups = 0;
    for &start in adj.keys() {
        if visited.contains(&start) {
            continue;
        }
        groups += 1;
        let mut stack = vec![start];
        visited.insert(start);
        while let Some(u) = stack.pop() {
            if let Some(neighbors) = adj.get(&u) {
                for &v in neighbors {
                    if !visited.contains(&v) {
                        visited.insert(v);
                        stack.push(v);
                    }
                }
            }
        }
    }
    println!("{}", groups);
}
