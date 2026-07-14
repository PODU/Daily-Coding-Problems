// Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
// BFS coloring over each component. Time: O(V+E). Space: O(V).
use std::collections::{BTreeMap, HashMap, VecDeque};

fn divide_teams(g: &BTreeMap<i32, Vec<i32>>) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut color: HashMap<i32, i32> = HashMap::new();
    for &s in g.keys() {
        if color.contains_key(&s) {
            continue;
        }
        color.insert(s, 0);
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            let cu = color[&u];
            for &v in &g[&u] {
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
    let mut a: Vec<i32> = color.iter().filter(|(_, &c)| c == 0).map(|(&n, _)| n).collect();
    let mut b: Vec<i32> = color.iter().filter(|(_, &c)| c == 1).map(|(&n, _)| n).collect();
    a.sort();
    b.sort();
    Some((a, b))
}

fn fmt_set(s: &[i32]) -> String {
    s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
}

fn main() {
    let mut g: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    g.insert(0, vec![3]);
    g.insert(1, vec![2]);
    g.insert(2, vec![1, 4]);
    g.insert(3, vec![0, 4, 5]);
    g.insert(4, vec![2, 3]);
    g.insert(5, vec![3]);
    match divide_teams(&g) {
        Some((a, b)) => println!("{{{}}} and {{{}}}", fmt_set(&a), fmt_set(&b)),
        None => println!("False"),
    }
    // {0, 1, 4, 5} and {2, 3}
}
