// Day 1184: Interleave ranked lists into one playlist respecting every ordering.
// Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
// Time O(V + E), Space O(V + E).
use std::collections::{HashMap, HashSet, VecDeque};

fn interleave(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut order: Vec<i32> = Vec::new();
    let mut known: HashSet<i32> = HashSet::new();
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indeg: HashMap<i32, i32> = HashMap::new();
    let mut edges: HashSet<(i32, i32)> = HashSet::new();

    for l in lists {
        for &v in l {
            if known.insert(v) {
                order.push(v);
                adj.entry(v).or_default();
                indeg.entry(v).or_insert(0);
            }
        }
        for w in l.windows(2) {
            let (u, x) = (w[0], w[1]);
            if u != x && edges.insert((u, x)) {
                adj.get_mut(&u).unwrap().push(x);
                *indeg.get_mut(&x).unwrap() += 1;
            }
        }
    }

    let mut q: VecDeque<i32> = order.iter().cloned().filter(|v| indeg[v] == 0).collect();
    let mut res = Vec::new();
    while let Some(v) = q.pop_front() {
        res.push(v);
        let neighbors = adj[&v].clone();
        for w in neighbors {
            let d = indeg.get_mut(&w).unwrap();
            *d -= 1;
            if *d == 0 {
                q.push_back(w);
            }
        }
    }
    res
}

fn main() {
    let lists = vec![vec![1, 7, 3], vec![2, 1, 6, 7, 9], vec![3, 9, 5]];
    let r = interleave(&lists);
    let strs: Vec<String> = r.iter().map(|x| x.to_string()).collect();
    println!("[{}]", strs.join(", ")); // [2, 1, 6, 7, 3, 9, 5]
}
