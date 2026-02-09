// Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
// Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

use std::collections::{BTreeMap, HashMap, VecDeque};

fn divide_teams(students: &BTreeMap<i32, Vec<i32>>) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut color: HashMap<i32, i32> = HashMap::new();
    for (&start, _) in students.iter() {
        if color.contains_key(&start) {
            continue;
        }
        color.insert(start, 0);
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(u) = q.pop_front() {
            for &v in &students[&u] {
                if !color.contains_key(&v) {
                    let c = color[&u] ^ 1;
                    color.insert(v, c);
                    q.push_back(v);
                } else if color[&v] == color[&u] {
                    return None;
                }
            }
        }
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (&n, _) in students.iter() {
        if color[&n] == 0 {
            a.push(n);
        } else {
            b.push(n);
        }
    }
    a.sort();
    b.sort();
    Some((a, b))
}

fn set_str(v: &[i32]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn fmt_result(res: Option<(Vec<i32>, Vec<i32>)>) -> String {
    match res {
        None => "False".to_string(),
        Some((a, b)) => format!("{} and {}", set_str(&a), set_str(&b)),
    }
}

fn main() {
    let s1: BTreeMap<i32, Vec<i32>> = [
        (0, vec![3]), (1, vec![2]), (2, vec![1, 4]),
        (3, vec![0, 4, 5]), (4, vec![2, 3]), (5, vec![3]),
    ].into_iter().collect();
    let s2: BTreeMap<i32, Vec<i32>> = [
        (0, vec![3]), (1, vec![2]), (2, vec![1, 3, 4]),
        (3, vec![0, 2, 4, 5]), (4, vec![2, 3]), (5, vec![3]),
    ].into_iter().collect();
    println!("{}", fmt_result(divide_teams(&s1)));
    println!("{}", fmt_result(divide_teams(&s2)));
}
