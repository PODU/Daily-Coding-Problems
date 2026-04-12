// Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
// Time: O(V+E), Space: O(V+E).
use std::collections::{BTreeMap, HashMap, VecDeque};

fn solve(students: &BTreeMap<i32, Vec<i32>>) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut color: HashMap<i32, i32> = HashMap::new();
    for (&start, _) in students.iter() {
        if color.contains_key(&start) {
            continue;
        }
        color.insert(start, 0);
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(u) = q.pop_front() {
            let mut nbrs = students.get(&u).cloned().unwrap_or_default();
            nbrs.sort();
            let cu = color[&u];
            for v in nbrs {
                if let Some(&cv) = color.get(&v) {
                    if cv == cu {
                        return None;
                    }
                } else {
                    color.insert(v, cu ^ 1);
                    q.push_back(v);
                }
            }
        }
    }
    let mut t0: Vec<i32> = color.iter().filter(|&(_, &c)| c == 0).map(|(&k, _)| k).collect();
    let mut t1: Vec<i32> = color.iter().filter(|&(_, &c)| c == 1).map(|(&k, _)| k).collect();
    t0.sort();
    t1.sort();
    Some((t0, t1))
}

fn group(g: &[i32]) -> String {
    let parts: Vec<String> = g.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn main() {
    let s1: BTreeMap<i32, Vec<i32>> = [
        (0, vec![3]), (1, vec![2]), (2, vec![1, 4]),
        (3, vec![0, 4, 5]), (4, vec![2, 3]), (5, vec![3]),
    ].into_iter().collect();
    match solve(&s1) {
        Some((t0, t1)) => println!("{} and {}", group(&t0), group(&t1)),
        None => println!("False"),
    }

    let s2: BTreeMap<i32, Vec<i32>> = [
        (0, vec![3]), (1, vec![2]), (2, vec![1, 3, 4]),
        (3, vec![0, 2, 4, 5]), (4, vec![2, 3]), (5, vec![3]),
    ].into_iter().collect();
    match solve(&s2) {
        Some((t0, t1)) => println!("{} and {}", group(&t0), group(&t1)),
        None => println!("False"),
    }
}
