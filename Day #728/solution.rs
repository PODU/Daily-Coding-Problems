// Day 728: Split students into two teams so no enemies share a team.
// Approach: BFS 2-coloring (bipartite check). Returns two teams or None.
// Time: O(V + E), Space: O(V).
use std::collections::{BTreeMap, HashMap, VecDeque};

fn two_teams(students: &BTreeMap<i32, Vec<i32>>) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut color: HashMap<i32, i32> = HashMap::new();
    for (&s, _) in students {
        if color.contains_key(&s) {
            continue;
        }
        color.insert(s, 0);
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            let cu = color[&u];
            for &v in &students[&u] {
                match color.get(&v) {
                    None => { color.insert(v, cu ^ 1); q.push_back(v); }
                    Some(&cv) => { if cv == cu { return None; } }
                }
            }
        }
    }
    let a: Vec<i32> = students.keys().cloned().filter(|k| color[k] == 0).collect();
    let b: Vec<i32> = students.keys().cloned().filter(|k| color[k] == 1).collect();
    Some((a, b))
}

fn set_str(v: &[i32]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", s.join(", "))
}

fn show(students: BTreeMap<i32, Vec<i32>>) {
    match two_teams(&students) {
        None => println!("False"),
        Some((a, b)) => println!("{} and {}", set_str(&a), set_str(&b)),
    }
}

fn main() {
    show(BTreeMap::from([(0, vec![3]), (1, vec![2]), (2, vec![1, 4]),
        (3, vec![0, 4, 5]), (4, vec![2, 3]), (5, vec![3])]));
    show(BTreeMap::from([(0, vec![3]), (1, vec![2]), (2, vec![1, 3, 4]),
        (3, vec![0, 2, 4, 5]), (4, vec![2, 3]), (5, vec![3])]));
}
